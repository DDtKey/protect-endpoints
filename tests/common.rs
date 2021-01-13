use rocket::http::hyper::header::AUTHORIZATION;
use rocket::local::asynchronous::LocalResponse;
use rocket::Request;
use std::fmt::{Display, Formatter};

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

#[derive(PartialEq, Clone)]
pub enum Role {
    ADMIN,
    MANAGER,
}

pub async fn extract(req: &mut Request<'_>) -> Option<Vec<String>> {
    let auth_headers: Vec<&str> = req.headers().get(AUTHORIZATION.as_str()).collect();

    auth_headers
        .get(0)
        .map(|h| h.split(",").map(str::to_string).collect::<Vec<String>>())
}

pub async fn enum_extract(req: &mut Request<'_>) -> Option<Vec<Role>> {
    let auth_headers: Vec<&str> = req.headers().get(AUTHORIZATION.as_str()).collect();

    auth_headers
        .get(0)
        .map(|h| h.split(",").map(|name| name.into()).collect::<Vec<Role>>())
}

pub async fn test_body(resp: LocalResponse<'_>, expected_body: &str) {
    let body = resp.into_string().await.unwrap();
    assert_eq!(expected_body, &body);
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
