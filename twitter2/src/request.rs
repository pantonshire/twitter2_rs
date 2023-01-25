use std::{fmt, num::NonZeroU8, borrow::Cow};

use chrono::{DateTime, Utc};
use enumscribe::ScribeStaticStr;
use libshire::{sink::{SinkString, StrSink, FmtSink}, convert::result_elim, sink_fmt};
use serde_json::Value;

use crate::{user::UserId, tweet::{TweetId, Tweet}, AsyncClient, auth::AppAuth, client::{Error, Request, Method, ErrorRepr, ErrorKind}, limit::LimitInfo, response::Includes, request_data::FormData, request_options::{TweetPayloadExpansion, TweetField, UserField, MediaField}, timeline::PaginationToken};

pub struct UserTimeline {
    id: UserId,
    start_time: Option<DateTime<Utc>>,
    end_time: Option<DateTime<Utc>>,
    exclude_retweets: bool,
    exclude_replies: bool,
    max_results: Option<NonZeroU8>,
    pagination_token: Option<PaginationToken>,
    since_id: Option<TweetId>,
    until_id: Option<TweetId>,
    expansions: String,
    tweet_fields: String,
    user_fields: String,
    media_fields: String,
}

impl UserTimeline {
    #[inline]
    #[must_use]
    pub fn new(id: UserId) -> Self {
        Self {
            id,
            start_time: None,
            end_time: None,
            exclude_retweets: false,
            exclude_replies: false,
            max_results: None,
            pagination_token: None,
            since_id: None,
            until_id: None,
            expansions: String::new(),
            tweet_fields: String::new(),
            user_fields: String::new(),
            media_fields: String::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn start_time(self, start_time: DateTime<Utc>) -> Self {
        Self {
            start_time: Some(start_time),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn end_time(self, end_time: DateTime<Utc>) -> Self {
        Self {
            end_time: Some(end_time),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn exclude_retweets(self) -> Self {
        Self {
            exclude_retweets: true,
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn exclude_replies(self) -> Self {
        Self {
            exclude_replies: true,
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn max_results(self, max_results: u8) -> Self {
        let max_results = max_results.clamp(5, 100);
        Self {
            max_results: Some(NonZeroU8::new(max_results).unwrap()),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn pagination_token(self, pagination_token: PaginationToken) -> Self {
        Self {
            pagination_token: Some(pagination_token),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn since_id(self, since_id: TweetId) -> Self {
        Self {
            since_id: Some(since_id),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn until_id(self, until_id: TweetId) -> Self {
        Self {
            until_id: Some(until_id),
            ..self
        }
    }

    // FIXME: use a decidated expansion type for each endpoint, since different endpoints allow
    // different expansions (even when they have the same payload type).
    #[inline]
    #[must_use]
    pub fn expansions<I>(self, expansions: I) -> Self
    where
        I: IntoIterator<Item = TweetPayloadExpansion>,
    {
        Self {
            expansions: scribe_comma_separated(expansions),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn tweet_fields<I>(self, tweet_fields: I) -> Self
    where
        I: IntoIterator<Item = TweetField>,
    {
        Self {
            tweet_fields: scribe_comma_separated(tweet_fields),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn user_fields<I>(self, user_fields: I) -> Self
    where
        I: IntoIterator<Item = UserField>,
    {
        Self {
            user_fields: scribe_comma_separated(user_fields),
            ..self
        }
    }

    #[inline]
    #[must_use]
    pub fn media_fields<I>(self, media_fields: I) -> Self
    where
        I: IntoIterator<Item = MediaField>,
    {
        Self {
            media_fields: scribe_comma_separated(media_fields),
            ..self
        }
    }

    pub async fn execute<A>(&self, client: AsyncClient<A>) -> Result<UserTimelineResponse, Error>
    where
        A: AppAuth,
    {
        let mut params = Vec::new();

        if let Some(start_time) = self.start_time {
            params.push((
                Cow::Borrowed("start_time"),
                Cow::Owned(start_time.to_rfc3339())
            ));
        }

        if let Some(end_time) = self.end_time {
            params.push((
                Cow::Borrowed("start_time"),
                Cow::Owned(end_time.to_rfc3339())
            ));
        }

        let excludes = match (self.exclude_retweets, self.exclude_replies) {
            (true, true) => Some("retweets,replies"),
            (true, false) => Some("retweets"),
            (false, true) => Some("replies"),
            (false, false) => None,
        };

        if let Some(excludes) = excludes {
            params.push((
                Cow::Borrowed("exclude"),
                Cow::Borrowed(excludes)
            ));
        }

        if let Some(max_results) = self.max_results {
            params.push((
                Cow::Borrowed("max_results"),
                Cow::Owned(format!("{}", max_results))
            ));
        }

        if let Some(pagination_token) = self.pagination_token.as_ref() {
            params.push((
                Cow::Borrowed("pagination_token"),
                Cow::Borrowed(&pagination_token.0)
            ))
        }

        if let Some(since_id) = self.since_id {
            params.push((
                Cow::Borrowed("since_id"),
                Cow::Owned(format!("{}", since_id))
            ));
        }

        if let Some(until_id) = self.until_id {
            params.push((
                Cow::Borrowed("until_id"),
                Cow::Owned(format!("{}", until_id))
            ));
        }

        if !self.expansions.is_empty() {
            params.push((
                Cow::Borrowed("expansions"),
                Cow::Borrowed(&self.expansions)
            ));
        }

        if !self.tweet_fields.is_empty() {
            params.push((
                Cow::Borrowed("tweet.fields"),
                Cow::Borrowed(&self.tweet_fields)
            ));
        }

        if !self.user_fields.is_empty() {
            params.push((
                Cow::Borrowed("user.fields"),
                Cow::Borrowed(&self.user_fields)
            ));
        }

        if !self.media_fields.is_empty() {
            params.push((
                Cow::Borrowed("media.fields"),
                Cow::Borrowed(&self.media_fields)
            ));
        }

        let (mut response, limit_info)
            = client.apiv2_request::<_, Box<[Tweet]>>(Request::new_with_data(
                Method::Get,
                &format!("https://api.twitter.com/2/users/{}/tweets", self.id),
                FormData::new(&params)
            )).await?;

        let tweets = response
            .data
            .ok_or_else(|| ErrorRepr {
                kind: ErrorKind::NoData,
                limit_info: Some(limit_info.clone()),
            }.boxed())?;

        let previous_token = match response.meta.remove("previous_token") {
            Some(Value::String(previous_token)) => {
                Some(PaginationToken(previous_token.into_boxed_str()))
            },
            _ => None,
        }; 

        let next_token = match response.meta.remove("next_token") {
            Some(Value::String(next_token)) => {
                Some(PaginationToken(next_token.into_boxed_str()))
            },
            _ => None,
        };

        Ok(UserTimelineResponse {
            tweets,
            includes: response.includes,
            previous_token,
            next_token,
            limit_info,
        })
    }
}

#[derive(Debug)]
pub struct UserTimelineResponse {
    pub tweets: Box<[Tweet]>,
    pub includes: Includes,
    pub previous_token: Option<PaginationToken>,
    pub next_token: Option<PaginationToken>,
    pub limit_info: LimitInfo,
}

fn scribe_comma_separated<T, I>(iter: I) -> String
where
    T: ScribeStaticStr,
    I: IntoIterator<Item = T>,
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
