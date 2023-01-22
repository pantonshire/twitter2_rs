use std::{borrow::Cow, str, sync::Arc, time::Duration, fmt};

use enumscribe::ScribeStaticStr;
use libshire::{
    encoding::url::{percent_decode_utf8, percent_encode, FormDecode},
    sink::{FmtSink, sink_fmt, SinkString, StrSink},
    convert::result_elim
};
use reqwest::{header::{HeaderValue, AUTHORIZATION}, StatusCode};
use serde::Deserialize;

use crate::{
    auth::{oauth10a::OAuth10aRequest, Auth, OAuth10a, AppAuth, UserAuth},
    response::{ApiV2Response, ResponseError, Includes, UserResponse},
    limit::LimitInfo,
    user::UserId,
    model::{PayloadUserModel, IncludedTweetModel, FromResponseError},
    request_data::{FormData, RequestData, QueryData}
};

#[derive(Clone)]
pub struct AsyncClient<A> {
    http_client: reqwest::Client,
    auth: Arc<A>,
}

impl<A: Auth> AsyncClient<A> {
    pub fn new(auth: A, timeout: Option<Duration>) -> Result<Self, reqwest::Error> {
        let builder = reqwest::Client::builder()
            .min_tls_version(reqwest::tls::Version::TLS_1_2)
            .https_only(true);

        let builder = match timeout {
            Some(timeout) => builder.timeout(timeout),
            None => builder,
        };

        let http_client = builder.build()?;

        Ok(Self {
            http_client,
            auth: Arc::new(auth),
        })
    }

    /// Consumes this client and returns a new client using the given authentication credentials.
    pub fn reauthenticate<T: Auth>(self, auth: T) -> AsyncClient<T> {
        AsyncClient {
            http_client: self.http_client,
            auth: Arc::new(auth),
        }
    }

    /// Create a new client which uses different authentication credentials, but uses the same HTTP
    /// connection pool as this client.
    pub fn clone_reauthenticate<T: Auth>(&self, auth: T) -> AsyncClient<T> {
        AsyncClient {
            http_client: self.http_client.clone(),
            auth: Arc::new(auth),
        }
    }

    async fn raw_request<'req, ReqData>(
        &self,
        request: Request<'req, ReqData>,
    ) -> Result<(reqwest::Response, LimitInfo), Error>
    where
        ReqData: RequestData,
    {
        let auth_header = {
            let auth_string = self.auth.auth_header(&request);
            // FIXME: might be better to just panic if this fails
            let mut auth_header = HeaderValue::from_str(&auth_string)
                .map_err(|_| ErrorRepr {
                    kind: ErrorKind::BadAuthHeader,
                    limit_info: None,
                }.boxed())?;
            auth_header.set_sensitive(true);
            auth_header
        };

        let request = {
            let builder = self
                .http_client
                .request(request.method.to_reqwest_method(), request.base_url)
                .header(AUTHORIZATION, auth_header);

            request
                .data
                .build_http_request(builder)
                .map_err(|err| ErrorRepr {
                    kind: ErrorKind::Transfer(err),
                    limit_info: None,
                }.boxed())?
        };

        self.http_client
            .execute(request)
            .await
            .map_err(|err| ErrorRepr {
                kind: ErrorKind::Transfer(err),
                limit_info: None,
            }.boxed())
            .map(|resp| {
                let limit_info = LimitInfo::from_headers(resp.headers());
                (resp, limit_info)
            })
    }
}

impl<A: AppAuth> AsyncClient<A> {
    pub async fn lookup_users<User, I>(
        &self,
        ids: I
    ) -> Result<(Box<[User]>, LimitInfo), Error>
    where
        User: PayloadUserModel,
        I: IntoIterator<Item = UserId>,
    {
        const ENDPOINT: &str = "https://api.twitter.com/2/users";

        let expansions = scribe_comma_separated(User::REQUIRED_EXPANSIONS);
        let tweet_fields = scribe_comma_separated(User::Tweet::REQUIRED_FIELDS);
        let user_fields = scribe_comma_separated(User::REQUIRED_FIELDS);

        let ids = fmt_comma_separated(ids.into_iter().map(|id| id.0));

        let (users, includes, limit_info)
            = self.apiv2_request::<_, Vec<UserResponse>>(Request::new_with_data(
                Method::Get,
                ENDPOINT,
                QueryData::new(&[
                    ("ids", &ids),
                    ("expansions", &expansions),
                    ("tweet.fields", &tweet_fields),
                    ("user.fields", &user_fields),
                ])
            )).await?;

        let tweets = if User::Tweet::SHOULD_DESERIALIZE {
            includes
                .tweets
                .into_vec()
                .into_iter()
                .map(|tweet| User::Tweet::from_response(tweet))
                .collect::<Result<Box<[_]>, _>>()
                .map_err(|err| ErrorRepr {
                    kind: ErrorKind::DeserializeModel(err),
                    limit_info: Some(limit_info.clone()),
                }.boxed())?
        } else {
            Box::default()
        };

        let users = users
            .into_iter()
            .map(|user| User::from_response(user, &tweets))
            .collect::<Result<Box<[_]>, _>>()
            .map_err(|err| ErrorRepr {
                kind: ErrorKind::DeserializeModel(err),
                limit_info: Some(limit_info.clone()),
            }.boxed())?;

        Ok((users, limit_info))
    }

    async fn apiv2_request<'req, ReqData, RespData>(
        &self,
        request: Request<'req, ReqData>
    ) -> Result<(RespData, Includes, LimitInfo), Error>
    where
        ReqData: RequestData,
        RespData: for<'de> Deserialize<'de>,
    {
        let (resp, limit_info) = self.raw_request(request).await?;

        let status = resp.status();
        
        let body = resp
            .bytes()
            .await
            .map_err(|err| ErrorRepr {
                kind: ErrorKind::Transfer(err),
                limit_info: Some(limit_info.clone()),
            }.boxed())?;
        
        // Attempt to deserialise the response body from JSON.
        let apiv2_response = serde_json::from_slice::<ApiV2Response<RespData>>(&body)
            .map_err(|err| ErrorRepr {
                kind: ErrorKind::InvalidResponse(err),
                limit_info: Some(limit_info.clone()),
            }.boxed())?;

        // Return an error if we got a non-2XX HTTP response code or a non-empty errors list.
        if !status.is_success() || !apiv2_response.errors.is_empty() {
            return Err(ErrorRepr {
                kind: ErrorKind::ErrorResponse { status, errors: apiv2_response.errors },
                limit_info: Some(limit_info),
            }.boxed());
        }
        
        let data = apiv2_response
            .data
            .ok_or_else(|| ErrorRepr {
                kind: ErrorKind::NoData,
                limit_info: Some(limit_info.clone()),
            }.boxed())?;
        
        Ok((data, apiv2_response.includes, limit_info))
    }
}

impl<A: UserAuth> AsyncClient<A> {

}

impl AsyncClient<OAuth10a> {
    pub async fn get_request_token(
        &self,
        callback_url: &str,
    ) -> Result<(AsyncClient<OAuth10aRequest>, Box<str>), Error>
    {
        const ENDPOINT: &str = "https://api.twitter.com/oauth/request_token";

        let data = [("oauth_callback", callback_url)];

        // FIXME: return limit info
        let (response, limit_info) = self
            .raw_request(Request::new_with_data(
                Method::Post,
                ENDPOINT,
                FormData::new(&data),
            ))
            .await?;

        // FIXME: better error
        if !response.status().is_success() {
            return Err(ErrorRepr {
                kind: ErrorKind::Custom(
                    format!("{}", response.status()).into(),
                ),
                limit_info: Some(limit_info),
            }.boxed());
        }

        let body = response
            .bytes()
            .await
            .map_err(|err| ErrorRepr {
                kind: ErrorKind::Transfer(err),
                limit_info: Some(limit_info.clone()),
            }.boxed())?;

        let (request_token, request_token_secret) = {
            let (mut token, mut token_secret) = (None, None);

            for (key, val) in FormDecoder::new(&body) {
                match &*key {
                    "oauth_token" => {
                        token = Some(val);
                    }
                    "oauth_token_secret" => {
                        token_secret = Some(val);
                    }
                    _ => (),
                }
            }

            let token = token.ok_or_else(|| ErrorRepr {
                kind: ErrorKind::Custom("no oauth_token in response".into()),
                limit_info: Some(limit_info.clone()),
            }.boxed())?;

            let token_secret = token_secret.ok_or_else(|| ErrorRepr {
                kind: ErrorKind::Custom("no oauth_token_secret in response".into()),
                limit_info: Some(limit_info.clone()),
            }.boxed())?;

            (token, token_secret)
        };

        let redirect_url = format!(
            "https://api.twitter.com/oauth/authorize?oauth_token={}",
            percent_encode(&*request_token)
        )
        .into_boxed_str();

        let request_auth = OAuth10aRequest::new(
            self.auth
                .with_access_token(&request_token, &request_token_secret),
        );

        Ok((self.clone_reauthenticate(request_auth), redirect_url))
    }
}

impl AsyncClient<OAuth10aRequest> {
    pub async fn get_access_token(
        self,
        verifier: &str,
    ) -> Result<(Box<str>, Box<str>), Error>
    {
        const ENDPOINT: &str = "https://api.twitter.com/oauth/access_token";

        let data = [("oauth_verifier", verifier)];

        // FIXME: return limit info
        let (response, limit_info) = self
            .raw_request(Request::new_with_data(
                Method::Post,
                ENDPOINT,
                FormData::new(&data),
            ))
            .await?;

        // FIXME: better error
        if !response.status().is_success() {
            return Err(ErrorRepr {
                kind: ErrorKind::Custom(
                    format!("{}", response.status()).into(),
                ),
                limit_info: Some(limit_info),
            }.boxed());
        }

        let body = response
            .bytes()
            .await
            .map_err(|err| ErrorRepr {
                kind: ErrorKind::Transfer(err),
                limit_info: Some(limit_info.clone()),
            }.boxed())?;

        let (mut token, mut token_secret) = (None, None);

        for (key, val) in FormDecoder::new(&body) {
            match &*key {
                "oauth_token" => {
                    token = Some(val);
                }
                "oauth_token_secret" => {
                    token_secret = Some(val);
                }
                _ => (),
            }
        }

        let token = token.ok_or_else(|| ErrorRepr {
            kind: ErrorKind::Custom("no oauth_token in response".into()),
            limit_info: Some(limit_info.clone()),
        }.boxed())?.into();

        let token_secret = token_secret.ok_or_else(|| ErrorRepr {
            kind: ErrorKind::Custom("no oauth_token_secret in response".into()),
            limit_info: Some(limit_info.clone()),
        }.boxed())?.into();

        Ok((token, token_secret))
    }
}

// FIXME: move into libshire
struct FormDecoder<'a> {
    bytes: &'a [u8],
}

impl<'a> FormDecoder<'a> {
    fn new<T>(bytes: &'a T) -> Self
    where
        T: AsRef<[u8]> + ?Sized,
    {
        Self {
            bytes: bytes.as_ref(),
        }
    }
}

impl<'a> Iterator for FormDecoder<'a> {
    type Item = (Cow<'a, str>, Cow<'a, str>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.bytes.is_empty() {
            return None;
        }
        let (pair, remainder) = split_on_byte(self.bytes, b'&');
        self.bytes = remainder;
        let (key, val) = split_on_byte(pair, b'=');
        let key = percent_decode_utf8(key, FormDecode);
        let val = percent_decode_utf8(val, FormDecode);
        Some((key, val))
    }
}

fn split_on_byte(bytes: &[u8], delim: u8) -> (&[u8], &[u8]) {
    match bytes.iter().copied().position(|byte| byte == delim) {
        Some(index) => {
            // SAFETY:
            // `position` always returns a valid index into the slice, so `index < bytes.len()`
            // must hold. Therefore, `..index` is a valid range over the slice.
            let prefix = unsafe { bytes.get_unchecked(..index) };
            // SAFETY:
            // `index < bytes.len()` must hold as discussed above, so `index + 1 <= bytes.len()`
            // must also hold. Therefore, `(index + 1)..` is a valid range over the slice.
            let suffix = unsafe { bytes.get_unchecked((index + 1)..) };
            (prefix, suffix)
        }
        None => (bytes, &bytes[bytes.len()..]),
    }
}

#[cfg(test)]
mod tests {
    use super::FormDecoder;

    #[test]
    fn test_form_decoder() {
        let mut decoder = FormDecoder::new("foo=baa&lorem=robo+%F0%9F%A4%96&baz");
        assert_eq!(
            decoder.next().as_ref().map(|(k, v)| (&**k, &**v)),
            Some(("foo", "baa"))
        );
        assert_eq!(
            decoder.next().as_ref().map(|(k, v)| (&**k, &**v)),
            Some(("lorem", "robo 🤖"))
        );
        assert_eq!(
            decoder.next().as_ref().map(|(k, v)| (&**k, &**v)),
            Some(("baz", ""))
        );
        assert_eq!(decoder.next(), None);
    }
}

#[derive(ScribeStaticStr, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Method {
    #[enumscribe(str = "GET")]
    Get,
    #[enumscribe(str = "POST")]
    Post,
    #[enumscribe(str = "PUT")]
    Put,
    #[enumscribe(str = "DELETE")]
    Delete,
}

impl Method {
    pub fn as_str(self) -> &'static str {
        self.scribe()
    }

    fn to_reqwest_method(self) -> reqwest::Method {
        match self {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Delete => reqwest::Method::DELETE,
        }
    }
}

// FIXME: impl Display and Error
#[derive(Debug)]
pub struct Error {
    repr: Box<ErrorRepr>,
}

impl Error {
    pub fn kind(&self) -> &ErrorKind {
        &self.repr.kind
    }

    pub fn limit_info(&self) -> Option<&LimitInfo> {
        self.repr.limit_info.as_ref()
    }
}

#[derive(Debug)]
struct ErrorRepr {
    kind: ErrorKind,
    limit_info: Option<LimitInfo>,
}

impl ErrorRepr {
    fn boxed(self) -> Error {
        Error { repr: Box::new(self) }
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    BadAuthHeader,
    // FIXME: separate variant for each of the different `reqwest::Error` variants
    Transfer(reqwest::Error),
    InvalidResponse(serde_json::Error),
    ErrorResponse {
        status: StatusCode,
        errors: Box<[ResponseError]>,
    },
    NoData,
    DeserializeModel(FromResponseError),
    // FIXME: replace this temporary variant
    Custom(Cow<'static, str>),
}

pub struct Request<'a, D> {
    method: Method,
    base_url: &'a str,
    data: D,
}

impl<'a> Request<'a, ()> {
    pub fn new(method: Method, base_url: &'a str) -> Self {
        Self {
            method,
            base_url,
            data: (),
        }
    }
}

impl<'a, D> Request<'a, D> {
    pub fn new_with_data(method: Method, base_url: &'a str, data: D) -> Self {
        Self {
            method,
            base_url,
            data,
        }
    }

    pub(crate) fn method_str(&self) -> &str {
        self.method.as_str()
    }

    pub(crate) fn base_url(&self) -> &str {
        self.base_url
    }

    pub(crate) fn data(&self) -> &D {
        &self.data
    }
}

fn scribe_comma_separated<'a, T, I>(iter: I) -> String
where
    T: ScribeStaticStr + 'a,
    I: IntoIterator<Item = &'a T>,
{
    let iter = iter.into_iter().map(|t| t.scribe());
    let mut sink = SinkString::empty();
    result_elim(sink_comma_separated(&mut sink, iter));
    sink.0
}

fn fmt_comma_separated<T, I>(iter: I) -> String
where
    T: fmt::Display,
    I: IntoIterator<Item = T>,
{
    let mut sink = SinkString::empty();
    result_elim(sink_fmt_comma_separated(&mut sink, iter.into_iter()));
    sink.0
}

fn sink_comma_separated<'a, I, S>(sink: &mut S, iter: I) -> Result<(), S::Error>
where
    I: IntoIterator<Item = &'a str>,
    S: StrSink,
{
    let mut iter = iter.into_iter();
    if let Some(first) = iter.next() {
        sink.sink_str(first)?;
        for item in iter {
            sink.sink_char(',')?;
            sink.sink_str(item)?;
        }
    }
    Ok(())
}

fn sink_fmt_comma_separated<T, I, S>(sink: &mut S, iter: I) -> Result<(), S::Error>
where
    T: fmt::Display,
    I: IntoIterator<Item = T>,
    S: FmtSink,
{
    let mut iter = iter.into_iter();
    if let Some(first) = iter.next() {
        sink_fmt!(sink, "{}", first)?;
        for item in iter {
            sink.sink_char(',')?;
            sink_fmt!(sink, "{}", item)?;
        }
    }
    Ok(())
}
