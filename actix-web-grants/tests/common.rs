use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::{HeaderValue, AUTHORIZATION};
use actix_web::{test, Error};
use serde::Deserialize;
use std::str::FromStr;

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

#[derive(parse_display::Display, parse_display::FromStr, PartialEq, Clone)]
#[display(style = "SNAKE_CASE")]
#[allow(clippy::upper_case_acronyms)]
pub enum Role {
    ADMIN,
    MANAGER,
}

#[derive(parse_display::Display, parse_display::FromStr, PartialEq, Clone)]
#[display(style = "SNAKE_CASE")]
#[allow(clippy::upper_case_acronyms)]
pub enum Permission {
    READ,
    WRITE,
}

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
                .split(',')
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .ok_or_else(|| ErrorUnauthorized("Authorization header incorrect!"))
}

pub async fn enum_extract<T: FromStr>(req: &ServiceRequest) -> Result<Vec<T>, Error> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    auth_header
        .map(|header| {
            header
                .split(',')
                .filter_map(|name| T::from_str(name).ok())
                .collect::<Vec<T>>()
        })
        .ok_or_else(|| ErrorUnauthorized("Authorization header incorrect!"))
}

pub async fn test_body(resp: ServiceResponse, expected_body: &str) {
    let body = test::read_body(resp).await;

    assert_eq!(String::from_utf8(body.to_vec()).unwrap(), expected_body);
}

#[derive(Deserialize)]
pub struct NamePayload {
    pub name: Option<String>,
}
