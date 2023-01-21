use std::fmt;

use enumscribe::EnumDeserialize;
use libshire::strings::InliningString23;
use serde::Deserialize;

use crate::{
    entity::{Annotation, Tag, TweetMention, Url},
    id::IdU64,
    media::MediaKey,
};

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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
