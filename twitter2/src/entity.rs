use std::ops::{Range, RangeInclusive};

use libshire::strings::InliningString23;
use serde::Deserialize;

use crate::{media::MediaKey, user::UserId};

#[derive(Deserialize, Debug)]
pub struct Annotation {
    start: usize,
    #[serde(rename = "end")]
    end_inclusive: usize,
    probability: f64,
    #[serde(rename = "type")]
    annotation_type: InliningString23,
    normalized_text: InliningString23,
}

impl Annotation {
    pub fn range(&self) -> RangeInclusive<usize> {
        self.start..=self.end_inclusive
    }

    pub fn probability(&self) -> f64 {
        self.probability
    }

    pub fn annotation_type(&self) -> &str {
        &self.annotation_type
    }

    pub fn normalized_text(&self) -> &str {
        &self.normalized_text
    }
}

#[derive(Deserialize, Debug)]
pub struct Tag {
    start: usize,
    end: usize,
    tag: InliningString23,
}

impl Tag {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn tag(&self) -> &str {
        &self.tag
    }
}

#[derive(Deserialize, Debug)]
pub struct TweetMention {
    start: usize,
    end: usize,
    username: InliningString23,
    id: UserId,
}

impl TweetMention {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn id(&self) -> UserId {
        self.id
    }
}

#[derive(Deserialize, Debug)]
pub struct UserMention {
    start: usize,
    end: usize,
    username: InliningString23,
}

impl UserMention {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

#[derive(Deserialize, Debug)]
pub struct Url {
    start: usize,
    end: usize,
    url: Box<str>,
    expanded_url: Box<str>,
    display_url: Box<str>,
    media_key: Option<MediaKey>,
}

impl Url {
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// A URL to Twitter's "t.co" domain, which redirects to the original URL.
    pub fn t_co_url(&self) -> &str {
        &self.url
    }

    pub fn expanded_url(&self) -> &str {
        &self.expanded_url
    }

    pub fn display_url(&self) -> &str {
        &self.display_url
    }

    pub fn media_key(&self) -> Option<MediaKey> {
        self.media_key
    }
}
