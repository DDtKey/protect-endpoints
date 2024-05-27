use http_body_util::BodyExt;
use salvo::http::header::{HeaderValue, AUTHORIZATION};
use salvo::Request;
use salvo::Response;
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

pub async fn extract(req: &mut Request) -> Result<HashSet<String>, Response> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    Ok(auth_header
        .map(|header| header.split(',').map(str::to_string).collect())
        .unwrap())
}

pub async fn enum_extract(req: &mut Request) -> Result<HashSet<Role>, Response> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    Ok(auth_header
        .map(|header| header.split(',').map(|name| name.into()).collect())
        .unwrap())
}

pub async fn test_body(resp: Response, expected_body: &str) {
    let body = resp
        .into_body()
        .collect()
        .await
        .expect("Failed to collect body")
        .to_bytes();

    assert_eq!(expected_body, &body);
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
