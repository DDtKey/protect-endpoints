use rocket::http::hyper::header::AUTHORIZATION;
use rocket::local::asynchronous::LocalResponse;
use rocket::Request;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

#[derive(PartialEq, Eq, Hash)]
pub enum Role {
    Admin,
    Manager,
}

pub async fn extract(req: &mut Request<'_>) -> Option<HashSet<String>> {
    let auth_headers: Vec<&str> = req.headers().get(AUTHORIZATION.as_str()).collect();

    auth_headers
        .first()
        .map(|h| h.split(',').map(str::to_string).collect())
}

pub async fn enum_extract(req: &mut Request<'_>) -> Option<HashSet<Role>> {
    let auth_headers: Vec<&str> = req.headers().get(AUTHORIZATION.as_str()).collect();

    auth_headers
        .first()
        .map(|h| h.split(',').map(|name| name.into()).collect())
}

pub async fn test_body(resp: LocalResponse<'_>, expected_body: &str) {
    let body = resp.into_string().await.unwrap();
    assert_eq!(expected_body, &body);
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "ADMIN"),
            Role::Manager => write!(f, "MANAGER"),
        }
    }
}

impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            "ADMIN" => Role::Admin,
            "MANAGER" => Role::Manager,
            _ => panic!("Unexpected enum value"),
        }
    }
}
