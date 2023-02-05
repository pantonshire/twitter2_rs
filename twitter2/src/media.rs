use std::{error, fmt, str};

use enumscribe::{EnumDeserialize, EnumSerialize};
use serde::{
    de::{Error, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct MediaKey {
    id: u64,
    prefix: u32,
}

impl MediaKey {
    pub const fn new(prefix: u32, id: u64) -> Self {
        Self { id, prefix }
    }

    pub const fn prefix(self) -> u32 {
        self.prefix
    }

    pub const fn id(self) -> u64 {
        self.id
    }
}

impl str::FromStr for MediaKey {
    type Err = MediaKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (prefix, id) = s.split_once('_').ok_or(MediaKeyError(()))?;
        let prefix = prefix.parse().map_err(|_| MediaKeyError(()))?;
        let id = id.parse().map_err(|_| MediaKeyError(()))?;
        Ok(Self::new(prefix, id))
    }
}

impl fmt::Display for MediaKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}_{}", self.prefix, self.id)
    }
}

impl Serialize for MediaKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MediaKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MediaKeyVisitor)
    }
}

struct MediaKeyVisitor;

impl<'de> Visitor<'de> for MediaKeyVisitor {
    type Value = MediaKey;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a media key string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        v.parse().map_err(|_| E::custom("invalid media key"))
    }
}

#[derive(Debug)]
pub struct MediaKeyError(());

impl fmt::Display for MediaKeyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid media key")
    }
}

impl error::Error for MediaKeyError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Media {
    pub media_key: MediaKey,
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

#[derive(EnumSerialize, EnumDeserialize, Debug)]
pub enum MediaType {
    #[enumscribe(str = "photo")]
    Photo,
    #[enumscribe(str = "animated_gif")]
    Gif,
    #[enumscribe(str = "video")]
    Video,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaPublicMetrics {
    view_count: u64,
}

impl MediaPublicMetrics {
    pub fn view_count(&self) -> u64 {
        self.view_count
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MediaVariant {
    content_type: Box<str>,
    url: Box<str>,
    bit_rate: Option<u64>,
}

impl MediaVariant {
    pub fn content_type(&self) -> &str {
        &self.content_type
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn bit_rate(&self) -> Option<u64> {
        self.bit_rate
    }
}
