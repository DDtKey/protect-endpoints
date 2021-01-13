use chrono::{Duration, Utc};
use jsonwebtoken::{self, DecodingKey, EncodingKey, Header, Validation};
use poem_grants::error::AccessError::UnauthorizedRequest;
use serde::{Deserialize, Serialize};

// Token lifetime and Secret key are hardcoded for clarity
const JWT_EXPIRATION_HOURS: i64 = 24;
const SECRET: &str = "SECRET";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub username: String,
    pub permissions: Vec<String>,
    exp: i64,
}

impl Claims {
    pub fn new(username: String, permissions: Vec<String>) -> Self {
        Self {
            username,
            permissions,
            exp: (Utc::now() + Duration::hours(JWT_EXPIRATION_HOURS)).timestamp(),
        }
    }
}

/// Create a json web token (JWT)
pub(crate) fn create_jwt(claims: Claims) -> poem::Result<String> {
    let encoding_key = EncodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::encode(&Header::default(), &claims, &encoding_key)
        .map_err(|_| UnauthorizedRequest.into()) // Just example, here should be the correct way to handle the error
}

/// Decode a json web token (JWT)
pub(crate) fn decode_jwt(token: &str) -> poem::Result<Claims> {
    let decoding_key = DecodingKey::from_secret(SECRET.as_bytes());
    jsonwebtoken::decode::<Claims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| UnauthorizedRequest.into()) // Just example, here should be the correct way to handle the error
}
