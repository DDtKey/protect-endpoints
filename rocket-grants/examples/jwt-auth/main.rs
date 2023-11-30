use std::collections::HashSet;
use rocket::http::hyper::header::AUTHORIZATION;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::Request;
use rocket_grants::GrantsFairing;
use serde::Deserialize;

use crate::claims::Claims;

mod claims;

#[rocket_grants::protect("OP_GET_SECURED_INFO")]
#[rocket::get("/api/admin")]
// For the user with permission `OP_GET_SECURED_INFO` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL (for generate you own token, use `/token` handler):
// ```sh
//  curl --location --request GET 'http://localhost:8080/api/admin' \
//  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IkxvcmVtLUlwc3VtIiwicGVybWlzc2lvbnMiOlsiT1BfR0VUX1NFQ1VSRURfSU5GTyJdLCJleHAiOjE5MjY2ODk3MTF9.vFZ6qYRhJ4KY3trXvIAnhTed8UXxCw2tCSB4Qz5D7So'
// ```
async fn permission_secured() -> Status {
    Status::Ok
}

#[rocket_grants::protect(any("ADMIN", "MANAGER"))]
#[rocket::get("/api/manager")]
// For the `ADMIN` or `MANAGER` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL (for generate you own token, use `/token` handler):
// ```sh
//  curl --location --request GET 'http://localhost:8080/api/manager' \
//  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IkxvcmVtLUlwc3VtIiwicGVybWlzc2lvbnMiOlsiUk9MRV9NQU5BR0VSIl0sImV4cCI6MTkyNjY5MDYxN30.AihInANG_8gp5IZk5gak9-Sv_ankb740FfNepzhZojw'
// ```
async fn manager_secured() -> Status {
    Status::Ok
}

#[rocket::launch]
// Sample application with grant protection based on extracting by your custom function
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            rocket::routes![permission_secured, manager_secured, create_token],
        )
        .attach(GrantsFairing::with_extractor_fn(|req| Box::pin(extract_from_jwt(req))))
}

async fn extract_from_jwt(req: &mut Request<'_>) -> Option<HashSet<String>> {
    req.headers()
        .get(AUTHORIZATION.as_str())
        .next()
        .filter(|value| value.starts_with("Bearer "))
        .map(|value| &value[7..])
        .and_then(claims::decode_jwt)
        .map(|claims| claims.permissions)
}

// An additional handler for generating a token.
// Thus, you can try to create your own tokens and check the operation of the permissions system.
// cURL example:
// ```sh
//  curl --location --request POST 'http://localhost:8080/token' \
//   --header 'Content-Type: application/json' \
//   --data-raw '{
//       "username": "Lorem-Ipsum",
//       "permissions": ["OP_GET_SECURED_INFO"]
//   }'
// ```
#[rocket::post("/token", data = "<info>")]
pub async fn create_token(info: Json<UserPermissions>) -> Result<String, &'static str> {
    let user_info = info.0;
    // Create a JWT
    let claims = Claims::new(user_info.username, user_info.permissions);
    let jwt = claims::create_jwt(claims)?;

    // Return token for work with example handlers
    Ok(jwt)
}

#[derive(Deserialize)]
pub struct UserPermissions {
    pub username: String,
    pub permissions: HashSet<String>,
}
