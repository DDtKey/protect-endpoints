use crate::role::Role::{self, ADMIN};
use poem::listener::TcpListener;
use poem::{get, http::StatusCode, EndpointExt, Request, Response, Route, Server};
use poem_grants::permissions::{AuthDetails, RolesCheck};
use poem_grants::GrantsMiddleware;

mod role;

// `proc-macro` way require specify your type. It can be an import or a full path.
#[poem_grants::has_any_role("ADMIN", "role::Role::MANAGER", type = "Role")]
#[poem::handler]
// For the `ADMIN` or `MANAGER` - endpoint will give the HTTP status 200, otherwise - 403
async fn macro_secured() -> Response {
    Response::builder().status(StatusCode::OK).finish()
}

#[poem::handler]
// An example of programmable protection with custom type
async fn manual_secure(details: AuthDetails<Role>) -> Response {
    if details.has_role(&Role::ADMIN) {
        return Response::builder()
            .status(StatusCode::OK)
            .body("Hello Admin!");
    }
    Response::builder().status(StatusCode::OK).body("Hello!")
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/manual", get(manual_secure))
        .at("/macro_secured", get(macro_secured))
        .with(GrantsMiddleware::with_extractor(extract));

    Server::new(TcpListener::bind("127.0.0.1:8082"))
        .run(app)
        .await
}

// You can specify any of your own type (`PartialEq` + `Clone`) for the return type wrapped in a vector: poem::Result<Vec<YOUR_TYPE_HERE>>
async fn extract(_req: &mut Request) -> poem::Result<Vec<Role>> {
    // Here is a place for your code to get user permissions/grants/permissions from a request
    // For example from a token or database

    // Stub example
    Ok(vec![Role::ADMIN])
}
