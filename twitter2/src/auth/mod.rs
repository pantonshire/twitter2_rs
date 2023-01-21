pub mod bearer_token;
pub mod oauth10a;

pub use bearer_token::BearerToken;
pub use oauth10a::OAuth10a;

use std::borrow::Cow;

use crate::client::{Request, RequestData};

pub trait Auth: sealed::Sealed {
    fn auth_header<D: RequestData>(&self, request: &Request<D>) -> Cow<str>;
}

pub trait AppAuth: Auth {}

pub trait UserAuth: AppAuth {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::bearer_token::BearerToken {}
    impl Sealed for super::oauth10a::OAuth10a {}
    impl Sealed for super::oauth10a::OAuth10aRequest {}
}
