use chrono::Utc;
use jsonwebtoken::{EncodingKey, Header, TokenData, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use actix_web::web;
use crate::services::users::UserAppState;
use crate::entity::users::Users;
use bson::{Bson, Document};

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Serialize, Deserialize)]
pub struct UserToken {
    pub iat: i64,
    pub exp: i64,
    pub user: String,
    pub id : String
}

impl UserToken {
    pub fn generate_token(email: String, id: String) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: email,
            id
        };
        let key = b"secret";
        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(key)).unwrap()
    }

    pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
        let key = b"secret";
        jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(key), &Validation::default())
    }



}