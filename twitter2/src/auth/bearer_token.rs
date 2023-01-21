use std::borrow::Cow;

use crate::client::{Request, RequestData};

use super::{AppAuth, Auth};

#[derive(Clone)]
pub struct BearerToken {
    auth_header: Box<str>,
}

impl BearerToken {
    pub fn new<S>(token: S) -> Self
    where
        S: AsRef<str>,
    {
        let bearer_token = token.as_ref();

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
