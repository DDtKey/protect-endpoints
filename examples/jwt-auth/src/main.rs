use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::{get, post, web, EndpointExt, Response, Route, Server};
use serde::Deserialize;

use crate::claims::Claims;
use crate::jwt_middleware::JwtMiddleware;

mod claims;
mod jwt_middleware;

#[poem_grants::has_permissions("OP_GET_SECURED_INFO")]
#[poem::handler]
// For the user with permission `OP_GET_SECURED_INFO` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL (for generate you own token, use `/token` handler):
// ```sh
//  curl --location --request GET 'http://localhost:8080/api/admin' \
//  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IkxvcmVtLUlwc3VtIiwicGVybWlzc2lvbnMiOlsiT1BfR0VUX1NFQ1VSRURfSU5GTyJdLCJleHAiOjE5MjY2ODk3MTF9.vFZ6qYRhJ4KY3trXvIAnhTed8UXxCw2tCSB4Qz5D7So'
// ```
async fn permission_secured() -> Response {
    Response::builder().status(StatusCode::OK).finish()
}

#[poem_grants::has_any_role("ADMIN", "MANAGER")]
#[poem::handler]
// For the `ADMIN` or `MANAGER` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL (for generate you own token, use `/token` handler):
// ```sh
//  curl --location --request GET 'http://localhost:8080/api/manager' \
//  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IkxvcmVtLUlwc3VtIiwicGVybWlzc2lvbnMiOlsiUk9MRV9NQU5BR0VSIl0sImV4cCI6MTkyNjY5MDYxN30.AihInANG_8gp5IZk5gak9-Sv_ankb740FfNepzhZojw'
// ```
async fn manager_secured() -> Response {
    Response::builder().status(StatusCode::OK).finish()
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::io::Error> {
    let api = Route::new()
        .at("/admin", get(permission_secured))
        .at("/manager", get(manager_secured))
        .with(JwtMiddleware);

    let app = Route::new()
        .at("/token", post(create_token))
        .nest("/api", api);

    Server::new(TcpListener::bind("127.0.0.1:8080"))
        .run(app)
        .await
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
#[poem::handler]
pub async fn create_token(info: web::Json<UserPermissions>) -> poem::Result<String> {
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
    pub permissions: Vec<String>,
}
