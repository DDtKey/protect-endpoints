use actix_web::dev::ServiceRequest;
use actix_web::{get, post, web, App, Error, HttpResponse, HttpServer};

use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::Deserialize;

use actix_web_grants::proc_macro::{has_any_role, has_permissions};
// Used for integration with `actix-web-httpauth`
use actix_web_grants::permissions::AttachPermissions;

use crate::claims::Claims;

mod claims;

#[get("/admin")]
#[has_permissions("OP_GET_SECURED_INFO")]
// For the user with permission `OP_GET_SECURED_INFO` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL (for generate you own token, use `/token` handler):
// ```sh
//  curl --location --request GET 'http://localhost:8080/api/admin' \
//  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IkxvcmVtLUlwc3VtIiwicGVybWlzc2lvbnMiOlsiT1BfR0VUX1NFQ1VSRURfSU5GTyJdLCJleHAiOjE5MjY2ODk3MTF9.vFZ6qYRhJ4KY3trXvIAnhTed8UXxCw2tCSB4Qz5D7So'
// ```
async fn permission_secured() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/manager")]
#[has_any_role("ADMIN", "MANAGER")]
// For the `ADMIN` or `MANAGER` - endpoint will give the HTTP status 200, otherwise - 403
// You can check via cURL (for generate you own token, use `/token` handler):
// ```sh
//  curl --location --request GET 'http://localhost:8080/api/manager' \
//  --header 'Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1c2VybmFtZSI6IkxvcmVtLUlwc3VtIiwicGVybWlzc2lvbnMiOlsiUk9MRV9NQU5BR0VSIl0sImV4cCI6MTkyNjY5MDYxN30.AihInANG_8gp5IZk5gak9-Sv_ankb740FfNepzhZojw'
// ```
async fn manager_secured() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    // We just get permissions from JWT
    let claims = claims::decode_jwt(credentials.token())?;
    req.attach(claims.permissions);
    Ok(req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = HttpAuthentication::bearer(validator);
        App::new().service(create_token).service(
            web::scope("/api")
                .wrap(auth)
                .service(permission_secured)
                .service(manager_secured),
        )
    })
    .bind("127.0.0.1:8080")?
    .workers(1)
    .run()
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
#[post("/token")]
pub async fn create_token(info: web::Json<UserPermissions>) -> Result<String, Error> {
    let user_info = info.into_inner();
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
