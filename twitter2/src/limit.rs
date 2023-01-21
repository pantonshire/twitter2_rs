use std::{num::NonZeroU64, str};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

const X_RATE_LIMIT_LIMIT: HeaderName = HeaderName::from_static("x-rate-limit-limit");
const X_RATE_LIMIT_REMAINING: HeaderName = HeaderName::from_static("x-rate-limit-remaining");
const X_RATE_LIMIT_RESET: HeaderName = HeaderName::from_static("x-rate-limit-reset");

#[derive(Clone, Debug)]
pub struct LimitInfo {
    limit: Option<NonZeroU64>,
    remaining: Option<NonZeroU64>,
    reset_secs: Option<NonZeroU64>,
}

impl LimitInfo {
    pub(crate) fn empty() -> Self {
        Self {
            limit: None,
            remaining: None,
            reset_secs: None,
        }
    }

    pub(crate) fn new(
        limit: Option<u64>,
        remaining: Option<u64>,
        reset_secs: Option<u64>
    ) -> Self {
        Self {
            limit: opt_u64_encode(limit),
            remaining: opt_u64_encode(remaining),
            reset_secs: opt_u64_encode(reset_secs),
        }
    }

    pub(crate) fn from_headers(headers: &HeaderMap) -> Self {
        let limit = headers.get(X_RATE_LIMIT_LIMIT).and_then(parse_int_header);
        let remaining = headers.get(X_RATE_LIMIT_REMAINING).and_then(parse_int_header);
        let reset_secs = headers.get(X_RATE_LIMIT_RESET).and_then(parse_int_header);
        Self::new(limit, remaining, reset_secs)
    }
    
    pub fn limit(&self) -> Option<u64> {
        opt_u64_decode(self.limit)
    }

    pub fn remaining(&self) -> Option<u64> {
        opt_u64_decode(self.remaining)
    }

    pub fn reset_seconds(&self) -> Option<u64> {
        opt_u64_decode(self.reset_secs)
    }
}

impl Default for LimitInfo {
    fn default() -> Self {
        Self::empty()
    }
}

fn opt_u64_encode(x: Option<u64>) -> Option<NonZeroU64> {
    x.and_then(|x| x.checked_add(1)).and_then(NonZeroU64::new)
}

fn opt_u64_decode(x: Option<NonZeroU64>) -> Option<u64> {
    x.map(|x| x.get() - 1)
}

fn parse_int_header(val: &HeaderValue) -> Option<u64> {
    str::from_utf8(val.as_bytes()).ok().and_then(|val| val.parse().ok())
}
