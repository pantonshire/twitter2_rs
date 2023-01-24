use std::borrow::Cow;

use libshire::encoding::url::percent_encode;
use reqwest::{RequestBuilder, header::{CONTENT_TYPE, HeaderValue}};
use serde::Serialize;

pub trait RequestData {
    fn has_params(&self) -> bool;
    fn for_each_param<'s, F: FnMut(&'s str, &'s str)>(&'s self, f: F);
    fn build_http_request(self, builder: RequestBuilder) -> reqwest::Result<reqwest::Request>;
}

impl RequestData for () {
    fn has_params(&self) -> bool {
        false
    }

    fn for_each_param<'s, F: FnMut(&'s str, &'s str)>(&'s self, _: F) {}

    fn build_http_request(self, builder: RequestBuilder) -> reqwest::Result<reqwest::Request> {
        builder.build()
    }
}
pub struct QueryData<'a> {
    params: &'a [(&'a str, &'a str)],
}

impl<'a> QueryData<'a> {
    pub fn new(params: &'a [(&'a str, &'a str)]) -> Self {
        Self { params }
    }
}

impl<'a> RequestData for QueryData<'a> {
    fn has_params(&self) -> bool {
        !self.params.is_empty()
    }

    fn for_each_param<'s, F: FnMut(&'s str, &'s str)>(&'s self, mut f: F) {
        for (key, val) in self.params {
            f(key, val)
        }
    }

    fn build_http_request(self, builder: RequestBuilder) -> reqwest::Result<reqwest::Request> {
        builder.query(self.params).build()
    }
}

pub struct FormData<'a> {
    params: &'a [(Cow<'a, str>, Cow<'a, str>)],
}

impl<'a> FormData<'a> {
    pub fn new(params: &'a [(Cow<'a, str>, Cow<'a, str>)]) -> Self {
        Self { params }
    }
}

impl<'a> RequestData for FormData<'a> {
    fn has_params(&self) -> bool {
        !self.params.is_empty()
    }

    fn for_each_param<'s, F: FnMut(&'s str, &'s str)>(&'s self, mut f: F) {
        for (key, val) in self.params {
            f(key, val)
        }
    }

    fn build_http_request(self, builder: RequestBuilder) -> reqwest::Result<reqwest::Request> {
        let mut buf = String::new();
        for (key, val) in self.params {
            if !buf.is_empty() {
                buf.push('&');
            }
            buf.push_str(&percent_encode(key.as_ref()));
            buf.push('=');
            buf.push_str(&percent_encode(val.as_ref()));
        }

        builder
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/x-www-form-urlencoded"),
            )
            .body(buf)
            .build()
    }
}

pub struct JsonData<'a, B: ?Sized> {
    json_body: &'a B,
}

impl<'a, B: ?Sized> JsonData<'a, B> {
    pub fn new(json_body: &'a B) -> Self {
        Self { json_body }
    }
}

impl<'a, B> RequestData for JsonData<'a, B>
where
    B: Serialize + ?Sized,
{
    fn has_params(&self) -> bool {
        false
    }

    fn for_each_param<'s, F: FnMut(&'s str, &'s str)>(&'s self, _: F) {}

    fn build_http_request(self, builder: RequestBuilder) -> reqwest::Result<reqwest::Request> {
        builder.json(self.json_body).build()
    }
}
