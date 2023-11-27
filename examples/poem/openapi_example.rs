use poem::listener::TcpListener;
use poem::{web, EndpointExt, Request, Route, Server};
use poem_grants::authorities::{AuthDetails, AuthoritiesCheck};
use poem_grants::GrantsMiddleware;
use poem_openapi::payload::PlainText;
use poem_openapi::{OpenApi, OpenApiService};

const ROLE_ADMIN: &str = "ROLE_ADMIN";
const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

struct Api;

#[poem_grants::open_api]
#[OpenApi]
impl Api {
    // An example of protection via `proc-macro`
    /// Documentation comment for `openapi` description works expected
    #[protect("OP_READ_ADMIN_INFO")]
    #[oai(path = "/admin", method = "get")]
    async fn macro_secured(&self) -> PlainText<String> {
        PlainText(ADMIN_RESPONSE.to_string())
    }

    // An example of programmable protection
    #[oai(path = "/", method = "get")]
    async fn manual_secure(&self, details: AuthDetails) -> PlainText<String> {
        if details.has_authority(ROLE_ADMIN) {
            return PlainText(ADMIN_RESPONSE.to_string());
        }
        PlainText(OTHER_RESPONSE.to_string())
    }

    // An example of protection via `proc-macro` with secure attribute
    #[protect("ROLE_ADMIN", expr = "*user_id == user.id")]
    #[oai(path = "/resource/:user_id", method = "get")]
    async fn secure_with_params(
        &self,
        user_id: web::Path<i32>,
        user: web::Data<&User>,
    ) -> PlainText<String> {
        PlainText(ADMIN_RESPONSE.to_string())
    }
}

struct User {
    id: i32,
}

#[tokio::main]
// Sample application with grant protection based on extracting by your custom function
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(Api, "Poem OpenApi + poem-grants", "1.0")
        .server("http://localhost:8081/");
    let app = Route::new().nest(
        "/",
        api_service.with(GrantsMiddleware::with_extractor(extract)),
    );
    Server::new(TcpListener::bind("127.0.0.1:8081"))
        .run(app)
        .await
}

// You can use both `&Request` and `&mut Request`
async fn extract(_req: &mut Request) -> poem::Result<Vec<String>> {
    // Here is a place for your code to get user permissions/roles/authorities from a request
    // For example from a token or database

    // Stub example
    Ok(vec![ROLE_ADMIN.to_string()])
}
