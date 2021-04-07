use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::{header::AUTHORIZATION, HeaderValue};
use actix_web::{test, Error};
use serde::Deserialize;

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

pub async fn extract(req: &ServiceRequest) -> Result<Vec<String>, Error> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    auth_header
        .map(|header| {
            header
                .split(",")
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .ok_or_else(|| ErrorUnauthorized("Authorization header incorrect!"))
}

pub async fn test_body(resp: ServiceResponse, expected_body: &str) {
    let body = test::read_body(resp).await;

    assert_eq!(expected_body, String::from_utf8(body.to_vec()).unwrap());
}

#[derive(Deserialize)]
pub struct NamePayload {
    pub name: Option<String>,
}
