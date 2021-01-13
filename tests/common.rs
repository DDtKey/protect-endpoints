use serde::{Deserialize, Serialize};

pub const ROLE_ADMIN: &str = "ROLE_ADMIN";
pub const ROLE_MANAGER: &str = "ROLE_MANAGER";

#[derive(Serialize, Deserialize)]
// Struct for passing authorities in a request
pub struct User {
    pub authorities: Vec<String>,
}
