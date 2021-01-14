use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::{header::AUTHORIZATION, HeaderValue};
use actix_web::Error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

#[derive(Serialize, Deserialize)]
// Struct for passing authorities in a request
pub struct User {
    pub authorities: Vec<String>,
}

pub async fn extract(req: Arc<ServiceRequest>) -> Result<Vec<String>, Error> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    let authorities = auth_header
        .map(serde_json::from_str::<User>)
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap().authorities)
        .ok_or_else(|| ErrorUnauthorized("Authorization header incorrect!"))?;

    Ok(authorities)
}
