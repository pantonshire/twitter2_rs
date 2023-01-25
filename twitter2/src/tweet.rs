use std::{fmt, str, num::ParseIntError};

use chrono::{DateTime, Utc};
use enumscribe::EnumDeserialize;
use libshire::strings::InliningString23;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{Annotation, Tag, TweetMention, Url},
    id::IdU64,
    media::MediaKey, user::UserId,
};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(from = "IdU64", into = "IdU64")]
pub struct TweetId(pub u64);

impl From<IdU64> for TweetId {
    fn from(IdU64(id): IdU64) -> Self {
        Self(id)
    }
}

impl From<TweetId> for IdU64 {
    fn from(TweetId(id): TweetId) -> Self {
        Self(id)
    }
}

impl fmt::Display for TweetId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <u64 as fmt::Display>::fmt(&self.0, f)
    }
}

impl str::FromStr for TweetId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

#[derive(Deserialize, Debug)]
pub struct Tweet {
    pub id: TweetId,
    pub text: Box<str>,
    #[serde(default)]
    pub attachments: TweetAttachments,
    pub author_id: Option<UserId>,
    // context_annotations:
    pub conversation_id: Option<TweetId>,
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub entities: TweetEntities,
    // geo:
    pub in_reply_to_user_id: Option<UserId>,
    // FIXME: parse language
    pub lang: Option<InliningString23>,
    // non_public_metrics:
    // organic_metrics:
    pub possibly_sensitive: Option<bool>,
    // promoted_metrics:
    pub public_metrics: Option<TweetPublicMetrics>,
    #[serde(default)]
    pub referenced_tweets: Box<[ReferencedTweet]>,
    pub reply_settings: Option<ReplySettings>,
    pub source: Option<InliningString23>,
    // withheld:
}

#[derive(Deserialize, Debug)]
pub struct ReferencedTweet {
    #[serde(rename = "type")]
    pub reference_type: ReferenceType,
    pub id: TweetId,
}

#[derive(Deserialize, Debug)]
pub struct TweetPublicMetrics {
    pub retweet_count: u64,
    pub reply_count: u64,
    pub like_count: u64,
    pub quote_count: u64,
}

#[derive(Deserialize, Default, Debug)]
pub struct TweetEntities {
    #[serde(default)]
    pub annotations: Box<[Annotation]>,
    #[serde(default)]
    pub cashtags: Box<[Tag]>,
    #[serde(default)]
    pub hashtags: Box<[Tag]>,
    #[serde(default)]
    pub mentions: Box<[TweetMention]>,
    #[serde(default)]
    pub urls: Box<[Url]>,
}

#[derive(Deserialize, Default, Debug)]
pub struct TweetAttachments {
    #[serde(default)]
    pub poll_ids: Box<[InliningString23]>,
    #[serde(default)]
    pub media_keys: Box<[MediaKey]>,
}

#[derive(EnumDeserialize, Debug)]
pub enum ReferenceType {
    #[enumscribe(str = "replied_to")]
    RepliedTo,
    #[enumscribe(str = "quoted")]
    Quoted,
    #[enumscribe(str = "retweeted")]
    Retweeted,
}

#[derive(EnumDeserialize, Debug)]
pub enum ReplySettings {
    #[enumscribe(str = "everyone")]
    Everyone,
    #[enumscribe(str = "mentioned_users")]
    MentionedUsers,
    #[enumscribe(str = "followers")]
    Followers,
}
