use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use axum::body::Body;
use axum::http::header::AUTHORIZATION;
use axum::http::{Request, StatusCode};
use axum::routing::get;
use axum::Router;
use axum_grants::authorities::{AuthDetails, AuthoritiesCheck};
use axum_grants::GrantsLayer;
use tower::ServiceExt;

const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

async fn different_body(details: AuthDetails) -> (StatusCode, &'static str) {
    if details.has_authority(ROLE_ADMIN) {
        return (StatusCode::OK, ADMIN_RESPONSE);
    }
    (StatusCode::OK, OTHER_RESPONSE)
}

async fn only_admin(details: AuthDetails) -> (StatusCode, &'static str) {
    if details.has_authority(ROLE_ADMIN) {
        return (StatusCode::OK, ADMIN_RESPONSE);
    }
    (StatusCode::FORBIDDEN, "")
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

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

async fn get_user_response(uri: &str, role: &str) -> axum::response::Response {
    let app = Router::new()
        .route("/", get(different_body))
        .route("/admin", get(only_admin))
        .layer(GrantsLayer::with_extractor(common::extract));

    app.oneshot(
        Request::builder()
            .header(AUTHORIZATION, role)
            .uri(uri)
            .body(Body::empty())
            .unwrap(),
    )
    .await
    .unwrap()
}
