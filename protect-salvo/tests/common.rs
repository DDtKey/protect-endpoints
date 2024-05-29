use http_body_util::BodyExt;
use salvo::http::header::{HeaderValue, AUTHORIZATION};
use salvo::http::{ReqBody, ResBody};
use salvo::macros::Extractible;
use serde::Deserialize;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

#[derive(Eq, PartialEq, Hash)]
#[allow(clippy::upper_case_acronyms)]
pub enum Role {
    ADMIN,
    MANAGER,
}

pub async fn extract(
    req: &mut salvo::hyper::Request<ReqBody>,
) -> Result<HashSet<String>, salvo::hyper::Response<ResBody>> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    auth_header
        .map(|header| header.split(',').map(str::to_string).collect())
        .ok_or_else(|| {
            salvo::hyper::Response::builder()
                .status(salvo::http::StatusCode::UNAUTHORIZED)
                .body(ResBody::None)
                .unwrap()
        })
}

pub async fn enum_extract(
    req: &mut salvo::hyper::Request<ReqBody>,
) -> Result<HashSet<Role>, salvo::hyper::Response<ResBody>> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    auth_header
        .map(|header| header.split(',').map(|name| name.into()).collect())
        .ok_or_else(|| {
            salvo::hyper::Response::builder()
                .status(salvo::http::StatusCode::UNAUTHORIZED)
                .body(ResBody::None)
                .unwrap()
        })
}

pub async fn test_body(mut resp: salvo::Response, expected_body: &str) {
    let body = resp
        .take_body()
        .collect()
        .await
        .expect("Failed to collect body")
        .to_bytes();

    assert_eq!(expected_body, &body);
}

#[derive(Deserialize, Extractible)]
#[salvo(extract(default_source(from = "query")))]
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
