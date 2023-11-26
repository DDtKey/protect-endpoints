use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use poem::http::header::AUTHORIZATION;
use poem::http::StatusCode;
use poem::test::{TestClient, TestResponse};
use poem::{EndpointExt, Response, Route};
use poem_grants::permissions::{AuthDetails, PermissionsCheck};
use poem_grants::GrantsMiddleware;

const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[poem::handler]
async fn different_body(details: AuthDetails) -> Response {
    if details.has_permission(ROLE_ADMIN) {
        return Response::builder()
            .status(StatusCode::OK)
            .body(ADMIN_RESPONSE);
    }
    Response::builder()
        .status(StatusCode::OK)
        .body(OTHER_RESPONSE)
}

#[poem::handler]
async fn only_admin(details: AuthDetails) -> Response {
    if details.has_permission(ROLE_ADMIN) {
        return Response::builder()
            .status(StatusCode::OK)
            .body(ADMIN_RESPONSE);
    }
    Response::builder().status(StatusCode::FORBIDDEN).finish()
}

#[tokio::test]
async fn test_different_bodies() {
    let admin_resp = get_user_response("/", ROLE_ADMIN).await;
    let manager_resp = get_user_response("/", ROLE_MANAGER).await;

    common::test_body(admin_resp, ADMIN_RESPONSE).await;
    common::test_body(manager_resp, OTHER_RESPONSE).await;
}

#[tokio::test]
async fn test_forbidden() {
    let test_admin = get_user_response("/admin", ROLE_ADMIN).await;
    let test_manager = get_user_response("/admin", ROLE_MANAGER).await;

    test_admin.assert_status_is_ok();
    test_manager.assert_status(StatusCode::FORBIDDEN);
}

async fn get_user_response(uri: &str, role: &str) -> TestResponse {
    let app = Route::new()
        .at("/", different_body)
        .at("/admin", only_admin)
        .with(GrantsMiddleware::with_extractor(common::extract));
    let cli = TestClient::new(app);

    cli.get(uri).header(AUTHORIZATION, role).send().await
}
