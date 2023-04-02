use crate::common;
use crate::common::Role::{self, ADMIN, MANAGER};
use poem::http::header::AUTHORIZATION;
use poem::http::StatusCode;
use poem::test::{TestClient, TestResponse};
use poem::{EndpointExt, Response, Route};
use poem_grants::{has_roles, GrantsMiddleware};

// Using imported custom type (in `use` section)
#[has_roles("ADMIN", type = "Role")]
#[poem::handler]
async fn imported_path_enum_secure() -> Response {
    Response::builder().status(StatusCode::OK).finish()
}

// Using a full path to a custom type (enum)
#[has_roles("crate::common::Role::ADMIN", type = "crate::common::Role")]
#[poem::handler]
async fn full_path_enum_secure() -> Response {
    Response::builder().status(StatusCode::OK).finish()
}

// Incorrect endpoint security without Type specification
#[has_roles("ADMIN")]
#[poem::handler]
async fn incorrect_enum_secure() -> Response {
    Response::builder().status(StatusCode::OK).finish()
}

#[tokio::test]
async fn test_http_response_for_imported_enum() {
    let test_admin = get_user_response("/imported_enum_secure", &ADMIN.to_string()).await;
    let test_manager = get_user_response("/imported_enum_secure", &MANAGER.to_string()).await;

    test_admin.assert_status_is_ok();
    test_manager.assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_http_response_for_full_path_enum() {
    let test_admin = get_user_response("/full_path_enum_secure", &ADMIN.to_string()).await;
    let test_manager = get_user_response("/full_path_enum_secure", &MANAGER.to_string()).await;

    test_admin.assert_status_is_ok();
    test_manager.assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_incorrect_http_response() {
    let test = get_user_response("/incorrect_enum_secure", &ADMIN.to_string()).await;
    test.assert_status(StatusCode::UNAUTHORIZED);
}

async fn get_user_response(uri: &str, role: &str) -> TestResponse {
    let app = Route::new()
        .at("/imported_enum_secure", imported_path_enum_secure)
        .at("/full_path_enum_secure", full_path_enum_secure)
        .at("/incorrect_enum_secure", incorrect_enum_secure)
        .with(GrantsMiddleware::with_extractor(common::enum_extract));
    let cli = TestClient::new(app);

    cli.get(uri).header(AUTHORIZATION, role).send().await
}
