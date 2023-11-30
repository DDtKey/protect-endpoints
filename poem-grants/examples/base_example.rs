use std::collections::HashSet;
use poem::http::StatusCode;
use poem::listener::TcpListener;
use poem::{get, web, EndpointExt, Request, Response, Route, Server};
use poem_grants::authorities::{AuthDetails, AuthoritiesCheck};
use poem_grants::GrantsMiddleware;

const ROLE_ADMIN: &str = "ROLE_ADMIN";
const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

// An example of protection via `proc-macro`
#[poem_grants::protect("OP_READ_ADMIN_INFO")]
#[poem::handler]
async fn macro_secured() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(ADMIN_RESPONSE)
}

// An example of programmable protection
#[poem::handler]
async fn manual_secure(details: AuthDetails) -> Response {
    if details.has_authority(ROLE_ADMIN) {
        return Response::builder()
            .status(StatusCode::OK)
            .body(ADMIN_RESPONSE);
    }
    Response::builder()
        .status(StatusCode::OK)
        .body(OTHER_RESPONSE)
}

// An example of protection via `proc-macro` with secure attribute
#[poem_grants::protect("ROLE_ADMIN", expr = "*user_id == user.id")]
#[poem::handler]
async fn secure_with_params(user_id: web::Path<i32>, user: web::Data<&User>) -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(ADMIN_RESPONSE)
}

struct User {
    id: i32,
}

#[tokio::main]
// Sample application with grant protection based on extracting by your custom function
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/", get(manual_secure))
        .at("/admin", get(macro_secured))
        .at("/resource/:user_id", get(secure_with_params))
        .with(GrantsMiddleware::with_extractor(extract));

    Server::new(TcpListener::bind("127.0.0.1:8081"))
        .run(app)
        .await
}

// You can use both `&Request` and `&mut Request`
async fn extract(_req: &mut Request) -> poem::Result<HashSet<String>> {
    // Here is a place for your code to get user permissions/roles/authorities from a request
    // For example from a token or database

    // Stub example
    Ok(HashSet::from([ROLE_ADMIN.to_string()]))
}
