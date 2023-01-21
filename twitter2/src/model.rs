use std::{error, fmt, rc::Rc, sync::Arc};

use crate::{
    media::MediaKey,
    request_options::{TweetField, UserField, UserPayloadExpansion},
    response::{TweetResponse, UserResponse},
    tweet::TweetId,
    user::UserId,
};

// TODO: remember to wrap included tweets in an `Arc`; the associated types e.g.
// `PayloadUserModel::Tweet` should be refcounted.

#[derive(Debug)]
pub enum FromResponseError {
    FieldMissing(&'static str),
    IncludeMissing(Include),
}

impl fmt::Display for FromResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FromResponseError::FieldMissing(name) => {
                write!(f, "missing field with name \"{}\"", name)
            }
            FromResponseError::IncludeMissing(include) => match include {
                Include::Tweet(id) => {
                    write!(f, "missing tweet expansion with id {}", id)
                }
                Include::User(id) => {
                    write!(f, "missing user expansion with id {}", id)
                }
                Include::Media(key) => {
                    write!(f, "missing media expansion with key {}", key)
                }
            },
        }
    }
}

impl error::Error for FromResponseError {}

#[derive(Debug)]
pub enum Include {
    Tweet(TweetId),
    User(UserId),
    Media(MediaKey),
}

// FIXME: PayloadTweetModel

pub trait PayloadUserModel: Sized {
    type Tweet: IncludedTweetModel + Clone;

    const REQUIRED_FIELDS: &'static [UserField];
    const REQUIRED_EXPANSIONS: &'static [UserPayloadExpansion];

    fn from_response(user: UserResponse, tweets: &[Self::Tweet])
        -> Result<Self, FromResponseError>;
}

pub trait IncludedTweetModel: Sized {
    const REQUIRED_FIELDS: &'static [TweetField];
    const SHOULD_DESERIALIZE: bool = true;

    fn id_matches(&self, id: TweetId) -> bool;
    fn from_response(tweet: TweetResponse) -> Result<Self, FromResponseError>;
}

impl<T: IncludedTweetModel> IncludedTweetModel for Rc<T> {
    const REQUIRED_FIELDS: &'static [TweetField] = T::REQUIRED_FIELDS;
    const SHOULD_DESERIALIZE: bool = T::SHOULD_DESERIALIZE;

    fn id_matches(&self, id: TweetId) -> bool {
        (&**self).id_matches(id)
    }

    fn from_response(tweet: TweetResponse) -> Result<Self, FromResponseError> {
        T::from_response(tweet).map(Rc::new)
    }
}

impl<T: IncludedTweetModel> IncludedTweetModel for Arc<T> {
    const REQUIRED_FIELDS: &'static [TweetField] = T::REQUIRED_FIELDS;
    const SHOULD_DESERIALIZE: bool = T::SHOULD_DESERIALIZE;

    fn id_matches(&self, id: TweetId) -> bool {
        (&**self).id_matches(id)
    }

    fn from_response(tweet: TweetResponse) -> Result<Self, FromResponseError> {
        T::from_response(tweet).map(Arc::new)
    }
}

impl IncludedTweetModel for () {
    const REQUIRED_FIELDS: &'static [TweetField] = &[];
    const SHOULD_DESERIALIZE: bool = false;

    fn id_matches(&self, _id: TweetId) -> bool {
        false
    }

    fn from_response(_tweet: TweetResponse) -> Result<Self, FromResponseError> {
        Ok(())
    }
}
