use enumscribe::ScribeStaticStr;

#[derive(ScribeStaticStr, Clone, Copy, Debug)]
pub enum TweetField {
    #[enumscribe(str = "attachments")]
    Attachments,
    #[enumscribe(str = "author_id")]
    AuthorId,
    #[enumscribe(str = "context_annotations")]
    ContextAnnotations,
    #[enumscribe(str = "conversation_id")]
    ConversationId,
    #[enumscribe(str = "created_at")]
    CreatedAt,
    #[enumscribe(str = "entities")]
    Entities,
    #[enumscribe(str = "geo")]
    Geo,
    #[enumscribe(str = "in_reply_to_user_id")]
    InReplyToUserId,
    #[enumscribe(str = "lang")]
    Lang,
    #[enumscribe(str = "non_public_metrics")]
    NonPublicMetrics,
    #[enumscribe(str = "organic_metrics")]
    OrganicMetrics,
    #[enumscribe(str = "possibly_sensitive")]
    PossiblySensitive,
    #[enumscribe(str = "promoted_metrics")]
    PromotedMetrics,
    #[enumscribe(str = "public_metrics")]
    PublicMetrics,
    #[enumscribe(str = "referenced_tweets")]
    ReferencedTweets,
    #[enumscribe(str = "reply_settings")]
    ReplySettings,
    #[enumscribe(str = "source")]
    Source,
    #[enumscribe(str = "withheld")]
    Withheld,
}

#[derive(ScribeStaticStr, Clone, Copy, Debug)]
pub enum UserField {
    #[enumscribe(str = "created_at")]
    CreatedAt,
    #[enumscribe(str = "description")]
    Description,
    #[enumscribe(str = "entities")]
    Entities,
    #[enumscribe(str = "location")]
    Location,
    #[enumscribe(str = "pinned_tweet_id")]
    PinnedTweetId,
    #[enumscribe(str = "profile_image_url")]
    ProfileImageUrl,
    #[enumscribe(str = "protected")]
    Protected,
    #[enumscribe(str = "public_metrics")]
    PublicMetrics,
    #[enumscribe(str = "url")]
    Url,
    #[enumscribe(str = "verified")]
    Verified,
    #[enumscribe(str = "withheld")]
    Withheld,
}

#[derive(ScribeStaticStr, Clone, Copy, Debug)]
pub enum MediaField {
    #[enumscribe(str = "url")]
    Url,
    #[enumscribe(str = "duration_ms")]
    DurationMs,
    #[enumscribe(str = "height")]
    Height,
    #[enumscribe(str = "non_public_metrics")]
    NonPublicMetrics,
    #[enumscribe(str = "organic_metrics")]
    OrganicMetrics,
    #[enumscribe(str = "preview_image_url")]
    PreviewImageUrl,
    #[enumscribe(str = "promoted_metrics")]
    PromotedMetrics,
    #[enumscribe(str = "public_metrics")]
    PublicMetrics,
    #[enumscribe(str = "width")]
    Width,
    #[enumscribe(str = "alt_text")]
    AltText,
    #[enumscribe(str = "variants")]
    Variants,
}

#[derive(ScribeStaticStr, Clone, Copy, Debug)]
pub enum TweetPayloadExpansion {
    #[enumscribe(str = "author_id")]
    AuthorId,
    #[enumscribe(str = "referenced_tweets.id")]
    ReferencedTweetsId,
    #[enumscribe(str = "in_reply_to_user_id")]
    InReplyToUserId,
    #[enumscribe(str = "attachments.media_keys")]
    AttachmentsMediaKeys,
    #[enumscribe(str = "attachments.poll_ids")]
    AttachmentsPollIds,
    #[enumscribe(str = "geo.place_id")]
    GeoPlaceId,
    #[enumscribe(str = "entities.mentions.username")]
    EntitiesMentionsUsername,
}

#[derive(ScribeStaticStr, Clone, Copy, Debug)]
pub enum UserPayloadExpansion {
    #[enumscribe(str = "pinned_tweet_id")]
    PinnedTweetId,
}

#[derive(ScribeStaticStr, Clone, Copy, Debug)]
pub enum IncludedReferencedTweetExpansion {
    #[enumscribe(str = "referenced_tweets.id.author_id")]
    AuthorId,
}
