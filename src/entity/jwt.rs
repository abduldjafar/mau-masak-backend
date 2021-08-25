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
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user: String,
}

impl UserToken {
    pub fn generate_token(email: String) -> String {
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let payload = UserToken {
            iat: now,
            exp: now + ONE_WEEK,
            user: email
        };
        let key = b"secret";
        jsonwebtoken::encode(&Header::default(), &payload, &EncodingKey::from_secret(key)).unwrap()
    }

    pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
        let key = b"secret";
        jsonwebtoken::decode::<UserToken>(&token, &DecodingKey::from_secret(key), &Validation::default())
    }

    pub fn verify_user(token_data: &TokenData<UserToken>, app_data: web::Data<UserAppState>) -> Result<String, String> {
        let action_user =  app_data.users_service_manager.find_by_email(data.email.to_string()).await;
        let result = web::block(move || action_user).await.unwrap();


        match result {
            Some(data) => {
                let password_from_body = data.pa
                let user: Users = bson::from_bson(Bson::Document(data)).unwrap();
            }
            None => {}
        }

        /**
        if User::is_valid_login_session(&token_data.claims, &pool.get().unwrap()) {
            Ok(token_data.claims.user.to_string())
        } else {
            Err("Invalid token".to_string())
        }
        **/
    }
}