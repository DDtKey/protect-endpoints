use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorUnauthorized;
use actix_web::http::header::{HeaderValue, AUTHORIZATION};
use actix_web::{test, Error};
use serde::Deserialize;
use std::fmt::{Display, Formatter};

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

#[derive(PartialEq, Clone)]
pub enum Role {
    ADMIN,
    MANAGER,
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
                .split(",")
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .ok_or_else(|| ErrorUnauthorized("Authorization header incorrect!"))
}

pub async fn enum_extract(req: &ServiceRequest) -> Result<Vec<Role>, Error> {
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
                .map(|name| name.into())
                .collect::<Vec<Role>>()
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

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::ADMIN => write!(f, "ADMIN"),
            Role::MANAGER => write!(f, "MANAGER"),
        }
    }
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "ADMIN" => Role::ADMIN,
            "MANAGER" => Role::MANAGER,
            _ => panic!("Unexpected enum value"),
        }
    }
}
