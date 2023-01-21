use std::time::Duration;

use twitter2::{model::{PayloadUserModel, FromResponseError}, request_options::{UserField, UserPayloadExpansion}, response::UserResponse, auth::{BearerToken, OAuth10a}, user::UserId};

const BEARER_TOKEN: &str = include_str!("../BEARER_TOKEN");
const OAUTH_CREDS: &str = include_str!("../OAUTH_10A");

#[derive(Debug)]
pub struct User {
    id: UserId,
    username: String,
    name: String,
    url: Option<String>,
}

impl PayloadUserModel for User {
    type Tweet = ();

    const REQUIRED_FIELDS: &'static [UserField] = &[
        UserField::Url,
    ];

    const REQUIRED_EXPANSIONS: &'static [UserPayloadExpansion] = &[

    ];

    fn from_response(user: UserResponse, tweets: &[Self::Tweet])
        -> Result<Self, FromResponseError>
    {
        Ok(User {
            id: user.id,
            username: user.username.into_string(),
            name: user.name.into_string(),
            url: user.url.map(String::from),
        })
    }
}

#[tokio::main]
async fn main() {
    let mut oauth_creds = OAUTH_CREDS
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            (!line.is_empty()).then_some(line)
        });

    let auth = OAuth10a::new(
        oauth_creds.next().unwrap(),
        oauth_creds.next().unwrap(),
        oauth_creds.next().unwrap(),
        oauth_creds.next().unwrap()
    );

    // let auth = BearerToken::new(BEARER_TOKEN.trim());
    
    let client = twitter2::client::AsyncClient::new(auth, Some(Duration::from_secs(30)))
        .unwrap();

    let (users, limit) = client.lookup_users::<User, _>([UserId(1030814512851681280)])
        .await
        .unwrap();

    println!("{:?}", limit);

    println!("{:?}", users);
}
