use std::fmt;

use serde::Deserialize;

use crate::{
    entity::{Tag, Url, UserMention},
    id::IdU64,
};

#[derive(Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
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

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Default, Debug)]
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

#[derive(Deserialize, Default, Debug)]
pub struct UserUrlEntities {
    #[serde(default)]
    urls: Box<[Url]>,
}

impl UserUrlEntities {
    pub fn urls(&self) -> &[Url] {
        &self.urls
    }
}

#[derive(Deserialize, Default, Debug)]
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
