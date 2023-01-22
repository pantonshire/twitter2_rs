use std::{borrow::Cow, collections::BTreeSet};

use base64::{engine::GeneralPurpose, Engine};
use chrono::Utc;
use hmac::{Hmac, Mac};
use libshire::{encoding::url::percent_encode, strings::CappedString};
use rand::{distributions::Alphanumeric, CryptoRng, Rng};
use sha1::Sha1;

use crate::{client::Request, request_data::RequestData};

use super::{AppAuth, Auth, UserAuth};

const NONCE_LEN: usize = 64;

/// A set of [OAuth 1.0a](https://oauth.net/core/1.0a/) credentials that can be used to
/// authenticate requests made on behalf of a specific user.
#[derive(Clone)]
pub struct OAuth10a {
    api_key_encoded: Box<str>,
    access_token_encoded: Box<str>,
    signing_key: Box<str>,
}

impl OAuth10a {
    /// Returns a new `OAuth10a` which can be used to authenticate requests made on behalf of a
    /// specific user.
    pub fn new(
        api_key: &str,
        api_key_secret: &str,
        access_token: &str,
        access_token_secret: &str,
    ) -> Self
    {
        let signing_key = {
            // Percent encode both components of the signing key.
            let api_key_secret_encoded = percent_encode(api_key_secret);
            let access_token_secret_encoded = percent_encode(access_token_secret);

            // Join the two components into a single string, separated by an ampersand.
            let cap = api_key_secret_encoded.len() + access_token_secret_encoded.len() + 1;
            let mut buf = String::with_capacity(cap);
            buf.push_str(&api_key_secret_encoded);
            buf.push('&');
            buf.push_str(&access_token_secret_encoded);
            buf.into_boxed_str()
        };

        Self {
            api_key_encoded: percent_encode(api_key).into(),
            access_token_encoded: percent_encode(access_token).into(),
            signing_key,
        }
    }

    /// Returns a new `OAuth10a` with the same API key pair but a different access token pair.
    #[must_use]
    pub fn with_access_token(&self, access_token: &str, access_token_secret: &str) -> Self {
        let signing_key = {
            // Find the position of the ampersand which separates the API key secret from the
            // access token secret.
            let sep_index = self
                .signing_key
                .find('&')
                .expect("the signing key should always contain an ampersand (&)");

            // Get the prefix of the original signing key containing the encoded API key secret and
            // the ampersand.
            let prefix = &self.signing_key[..(sep_index + 1)];

            let access_token_secret_encoded = percent_encode(access_token_secret);

            // Concatenate the prefix (which ends with an ampersand) with the percent encoded access
            // token secret.
            let mut buf = String::with_capacity(prefix.len() + access_token_secret_encoded.len());
            buf.push_str(prefix);
            buf.push_str(&access_token_secret_encoded);
            buf.into_boxed_str()
        };

        Self {
            api_key_encoded: self.api_key_encoded.clone(),
            access_token_encoded: percent_encode(access_token).into(),
            signing_key,
        }
    }

    fn parameter_string<D: RequestData>(
        &self,
        request: &Request<D>,
        nonce_encoded: &str,
        timestamp: i64,
    ) -> Box<str>
    {
        // FIXME: don't do this dynamic allocation & sorting if the request has no parameters
        let mut params = BTreeSet::<(Cow<str>, Cow<str>)>::new();

        params.insert((
            Cow::Borrowed("oauth_consumer_key"),
            Cow::Borrowed(&self.api_key_encoded),
        ));
        params.insert((Cow::Borrowed("oauth_nonce"), Cow::Borrowed(nonce_encoded)));
        params.insert((
            Cow::Borrowed("oauth_signature_method"),
            Cow::Borrowed("HMAC-SHA1"),
        ));
        params.insert((
            Cow::Borrowed("oauth_timestamp"),
            Cow::Owned(timestamp.to_string()),
        ));
        params.insert((
            Cow::Borrowed("oauth_token"),
            Cow::Borrowed(&self.access_token_encoded),
        ));
        params.insert((Cow::Borrowed("oauth_version"), Cow::Borrowed("1.0")));

        request.data().for_each_param(|key, val| {
            params.insert((percent_encode(key), percent_encode(val)));
        });

        let mut buf = String::new();
        for (key, val) in params {
            if !buf.is_empty() {
                buf.push('&');
            }
            buf.push_str(&key);
            buf.push('=');
            buf.push_str(&val);
        }

        buf.into_boxed_str()
    }

    fn signature_base<D: RequestData>(
        &self,
        request: &Request<D>,
        nonce_encoded: &str,
        timestamp: i64,
    ) -> Box<str>
    {
        let method = request.method_str();
        let base_url_encoded = percent_encode(request.base_url());
        let parameter_string = self.parameter_string(request, nonce_encoded, timestamp);
        let parameter_string_encoded = percent_encode(&*parameter_string);

        let cap = method.len() + base_url_encoded.len() + parameter_string_encoded.len() + 2;
        let mut buf = String::with_capacity(cap);
        buf.push_str(method);
        buf.push('&');
        buf.push_str(&base_url_encoded);
        buf.push('&');
        buf.push_str(&parameter_string_encoded);
        buf.into_boxed_str()
    }

    fn signature<D: RequestData>(
        &self,
        request: &Request<D>,
        nonce_encoded: &str,
        timestamp: i64,
    ) -> Box<str>
    {
        const BASE64_ENGINE: GeneralPurpose = base64::engine::general_purpose::STANDARD;

        let base = self.signature_base(request, nonce_encoded, timestamp);

        // Compute `hmac_sha1(signing_key, base)`.
        let signature_bytes = {
            let mut mac = Hmac::<Sha1>::new_from_slice(self.signing_key.as_bytes())
                .expect("the signing key should always be valid for HMAC-SHA1");
            mac.update(base.as_bytes());
            mac.finalize().into_bytes()
        };

        BASE64_ENGINE.encode(signature_bytes).into_boxed_str()
    }
}

impl Auth for OAuth10a {
    fn auth_header<D: RequestData>(&self, request: &Request<D>) -> Cow<str> {
        // The nonce is generated using only the characters 0..=9, A..=Z and a..=z, so it is
        // already percent-encoded.
        let nonce = gen_alphanumeric_nonce(&mut rand::thread_rng());
        let timestamp = Utc::now().timestamp();
        let signature = self.signature(request, &nonce, timestamp);

        Cow::Owned(format!(
            r#"OAuth oauth_consumer_key="{}", oauth_nonce="{}", oauth_signature="{}", oauth_signature_method="HMAC-SHA1", oauth_timestamp="{}", oauth_token="{}", oauth_version="1.0""#,
            self.api_key_encoded,
            nonce,
            percent_encode(&*signature),
            timestamp,
            self.access_token_encoded
        ))
    }
}

impl AppAuth for OAuth10a {}

impl UserAuth for OAuth10a {}

#[derive(Clone)]
pub struct OAuth10aRequest {
    inner: OAuth10a,
}

impl OAuth10aRequest {
    // Restricted to `pub(crate)` because the user should never create one of these directly; it
    // should only be possible to obtain one from `AsyncClient::<OAuth10a>::get_request_token`.
    pub(crate) fn new(auth: OAuth10a) -> Self {
        Self { inner: auth }
    }
}

impl Auth for OAuth10aRequest {
    fn auth_header<D: RequestData>(&self, request: &Request<D>) -> Cow<str> {
        self.inner.auth_header(request)
    }
}

fn gen_alphanumeric_nonce<R>(rng: &mut R) -> CappedString<NONCE_LEN>
where
    R: Rng + CryptoRng + ?Sized,
{
    let mut buf = CappedString::<NONCE_LEN>::empty();

    // Sample 64 random alphanumeric characters and push them into the buffer.
    for _ in 0..NONCE_LEN {
        buf.push_truncating(char::from(rng.sample(Alphanumeric)));
    }

    buf
}

#[cfg(test)]
mod tests {
    use crate::{client::{Method, Request}, request_data::FormData};

    use super::OAuth10a;

    #[test]
    fn test_signature() {
        // These are example credentials from the Twitter documentation :)
        let api_key = "xvz1evFS4wEEPTGEFPHBog";
        let api_key_secret = "kAcSOqF21Fu85e7zjz7ZN2U4ZRhfV3WpwPAoE3Z7kBw";
        let access_token = "370773112-GmHxMAgYyLbNEtIKZeRNFsMKPR9EyMZeS9weJAEb";
        let access_token_secret = "LswwdoUaIvS8ltyTt5jkRh4J50vUPVVHtR2YPi5kE";

        let auth = OAuth10a::new(api_key, api_key_secret, access_token, access_token_secret);

        let base_url = "https://api.twitter.com/1.1/statuses/update.json";
        let data = FormData::new(&[
            ("include_entities", "true"),
            (
                "status",
                "Hello Ladies + Gentlemen, a signed OAuth request!",
            ),
        ]);

        let request = Request::new_with_data(Method::Post, base_url, data);

        let nonce = "kYjzVBB8Y0ZFabxSWbWovY3uYSQ2pTgmZeNu2VS4cg";
        let timestamp = 1318622958;

        assert_eq!(
            &*auth.signature(&request, nonce, timestamp),
            "hCtSmYh+iHYCEqBWrE7C7hYmtUk="
        );
    }
}
