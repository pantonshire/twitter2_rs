use chrono::{DateTime, Utc};
use libshire::strings::InliningString23;
use serde::Deserialize;

use crate::{media::{MediaKey, MediaPublicMetrics, MediaType, MediaVariant}, user::{UserId, UserEntities, UserPublicMetrics}, tweet::{TweetId, TweetAttachments, TweetEntities, TweetPublicMetrics, ReferencedTweet, ReplySettings}};

#[derive(Deserialize, Debug)]
pub struct TweetResponse {
    pub id: TweetId,
    pub text: Box<str>,
    #[serde(default)]
    pub attachments: TweetAttachments,
    pub author_id: Option<UserId>,
    //context_annotations: 
    pub conversation_id: Option<TweetId>,
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default)]
    pub entities: TweetEntities,
    //geo: 
    pub in_reply_to_user_id: Option<UserId>,
    //TODO: parse language
    pub lang: Option<InliningString23>,
    //non_public_metrics: 
    //organic_metrics: 
    pub possibly_sensitive: Option<bool>,
    //promoted_metrics: 
    pub public_metrics: Option<TweetPublicMetrics>,
    #[serde(default)]
    pub referenced_tweets: Box<[ReferencedTweet]>, 
    pub reply_settings: Option<ReplySettings>,
    pub source: Option<InliningString23>,
    //withheld: 
}

#[derive(Deserialize, Debug)]
pub struct UserResponse {
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
    //withheld: 
}

#[derive(Deserialize, Debug)]
pub struct MediaResponse {
    pub key: MediaKey,
    #[serde(rename = "type")]
    pub media_type: MediaType,
    pub url: Option<Box<str>>,
    pub duration_ms: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    //non_public_metrics:
    //organic_metrics:
    pub preview_image_url: Option<Box<str>>,
    //promoted_metrics:
    pub public_metrics: Option<MediaPublicMetrics>,
    pub alt_text: Option<Box<str>>,
    #[serde(default)]
    pub variants: Box<[MediaVariant]>,
}
