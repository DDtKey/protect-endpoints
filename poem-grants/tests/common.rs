use poem::http::header::{HeaderValue, AUTHORIZATION};
use poem::test::TestResponse;
use poem::Request;
use serde::Deserialize;
use std::fmt::{Display, Formatter};

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

#[derive(PartialEq, Clone)]
#[allow(clippy::upper_case_acronyms)]
pub enum Role {
    ADMIN,
    MANAGER,
}

pub async fn extract(req: &mut Request) -> poem::Result<Vec<String>> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    Ok(auth_header
        .map(|header| {
            header
                .split(',')
                .map(str::to_string)
                .collect::<Vec<String>>()
        })
        .unwrap())
}

pub async fn enum_extract(req: &mut Request) -> poem::Result<Vec<Role>> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    Ok(auth_header
        .map(|header| {
            header
                .split(',')
                .map(|name| name.into())
                .collect::<Vec<Role>>()
        })
        .unwrap())
}

pub async fn test_body(resp: TestResponse, expected_body: &str) {
    let body = resp.0.into_body().into_string().await.unwrap();

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
