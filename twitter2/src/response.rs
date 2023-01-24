use serde::Deserialize;
use serde_json::{Value, Map};

use crate::{ media::Media, tweet::Tweet, user::User };

#[derive(Deserialize, Debug)]
pub(crate) struct ApiV2Response<T> {
    pub data: Option<T>,
    #[serde(default)]
    pub includes: Includes,
    #[serde(default)]
    pub errors: Box<[ResponseError]>,
    pub title: Option<Box<str>>,
    pub detail: Option<Box<str>>,
    #[serde(rename = "type")]
    pub response_type: Option<Box<str>>,
    pub status: Option<u16>,
    pub meta: Map<String, Value>,
}

#[derive(Deserialize, Debug)]
pub struct ResponseError {
    pub parameters: Option<ErrorParameters>,
    pub code: Option<u32>,
    pub message: Option<Box<str>>,
}

#[derive(Deserialize, Debug)]
pub struct ErrorParameters {
    #[serde(default)]
    pub expansions: Box<[Box<str>]>,
}

#[derive(Deserialize, Default, Debug)]
pub struct Includes {
    #[serde(default)]
    pub tweets: Box<[Tweet]>,
    #[serde(default)]
    pub users: Box<[User]>,
    #[serde(default)]
    pub media: Box<[Media]>,
}
