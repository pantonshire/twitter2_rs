use std::borrow::Cow;

use crate::{client::Request, request_data::RequestData};

use super::{AppAuth, Auth};

/// An app-only bearer token which can be used to authenticate requests made on behalf of a
/// [Twitter App](https://developer.twitter.com/en/docs/apps/overview). For example, this can be
/// used to retrieve public Tweets, but cannot be used to publish new Tweets.
/// 
/// For more information about bearer tokens, see the
/// [Twitter documentation](https://developer.twitter.com/en/docs/authentication/oauth-2-0/bearer-tokens).
/// 
/// If you want to make requests on behalf of a user rather than a Twitter App, see
/// [`OAuth10a`](crate::auth::oauth10a::OAuth10a).
#[derive(Clone)]
pub struct BearerToken {
    auth_header: Box<str>,
}

impl BearerToken {
    /// Returns a new `BearerToken` struct using the given app-only bearer token, which can be used
    /// to authenticate requests made on behalf of a
    /// [Twitter App](https://developer.twitter.com/en/docs/apps/overview).
    pub fn new<S>(token: S) -> Self
    where
        S: AsRef<str>,
    {
        let bearer_token = token.as_ref();

        // Create the Authorization header ahead-of-time, since it will be the same for every
        // request using this `BearerToken`.
        let auth_header = {
            const PREFIX: &str = "Bearer ";
            let mut buf = String::with_capacity(PREFIX.len() + bearer_token.len());
            buf.push_str(PREFIX);
            buf.push_str(bearer_token);
            buf.into_boxed_str()
        };

        Self { auth_header }
    }
}

impl Auth for BearerToken {
    fn auth_header<D: RequestData>(&self, _request: &Request<D>) -> Cow<str> {
        Cow::Borrowed(&self.auth_header)
    }
}

impl AppAuth for BearerToken {}
