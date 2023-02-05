use std::{fmt, str, num::ParseIntError};

use chrono::{DateTime, Utc};
use libshire::strings::InliningString23;
use serde::{Deserialize, Serialize};

use crate::{
    entity::{Tag, Url, UserMention},
    id::IdU64, tweet::TweetId,
};

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[serde(from = "IdU64", into = "IdU64")]
pub struct UserId(pub u64);

impl From<IdU64> for UserId {
    fn from(IdU64(id): IdU64) -> Self {
        Self(id)
    }
}

impl From<UserId> for IdU64 {
    fn from(UserId(id): UserId) -> Self {
        Self(id)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <u64 as fmt::Display>::fmt(&self.0, f)
    }
}

impl str::FromStr for UserId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map(Self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: UserId,
    pub name: InliningString23,
    pub username: InliningString23,
    pub created_at: Option<DateTime<Utc>>,
    pub description: Option<Box<str>>,
    pub entities: Option<UserEntities>,
    pub location: Option<Box<str>>,
    pub pinned_tweet_id: Option<TweetId>,
    pub profile_image_url: Option<Box<str>>,
    pub protected: Option<bool>,
    pub public_metrics: Option<UserPublicMetrics>,
    pub url: Option<Box<str>>,
    pub verified: Option<bool>,
    // withheld:
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserPublicMetrics {
    followers_count: u64,
    following_count: u64,
    tweet_count: u64,
    listed_count: u64,
}

impl UserPublicMetrics {
    pub fn followers_count(&self) -> u64 {
        self.followers_count
    }

    pub fn following_count(&self) -> u64 {
        self.following_count
    }

    pub fn tweet_count(&self) -> u64 {
        self.tweet_count
    }

    pub fn listed_count(&self) -> u64 {
        self.listed_count
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserEntities {
    #[serde(default)]
    url: UserUrlEntities,
    #[serde(default)]
    description: UserDescriptionEntities,
}

impl UserEntities {
    pub fn url(&self) -> &UserUrlEntities {
        &self.url
    }

    pub fn description(&self) -> &UserDescriptionEntities {
        &self.description
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserUrlEntities {
    #[serde(default)]
    urls: Box<[Url]>,
}

impl UserUrlEntities {
    pub fn urls(&self) -> &[Url] {
        &self.urls
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct UserDescriptionEntities {
    #[serde(default)]
    cashtags: Box<[Tag]>,
    #[serde(default)]
    hashtags: Box<[Tag]>,
    #[serde(default)]
    mentions: Box<[UserMention]>,
    #[serde(default)]
    urls: Box<[Url]>,
}

impl UserDescriptionEntities {
    pub fn cashtags(&self) -> &[Tag] {
        &self.cashtags
    }

    pub fn hashtags(&self) -> &[Tag] {
        &self.hashtags
    }

    pub fn mentions(&self) -> &[UserMention] {
        &self.mentions
    }

    pub fn urls(&self) -> &[Url] {
        &self.urls
    }
}
