use crate::common;
use crate::common::Role::{self, ADMIN, MANAGER};
use axum::body::Body;
use axum::http::header::AUTHORIZATION;
use axum::http::{Request, StatusCode};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use protect_axum::{protect, GrantsLayer};
use tower::ServiceExt;

// Using imported custom type (in `use` section)
#[protect("ADMIN", ty = "Role")]
async fn imported_path_enum_secure() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(().into())
        .unwrap()
}

// Using a full path to a custom type (enum)
#[protect("crate::common::Role::ADMIN", ty = "crate::common::Role")]
async fn full_path_enum_secure() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(().into())
        .unwrap()
}

// Incorrect endpoint security without Type specification
#[protect("ROLE_ADMIN")]
async fn incorrect_enum_secure() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(().into())
        .unwrap()
}

#[tokio::test]
async fn test_http_response_for_imported_enum() {
    let test_admin = get_user_response("/imported_enum_secure", &ADMIN.to_string()).await;
    let test_manager = get_user_response("/imported_enum_secure", &MANAGER.to_string()).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

#[tokio::test]
async fn test_http_response_for_full_path_enum() {
    let test_admin = get_user_response("/full_path_enum_secure", &ADMIN.to_string()).await;
    let test_manager = get_user_response("/full_path_enum_secure", &MANAGER.to_string()).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

#[tokio::test]
async fn test_incorrect_http_response() {
    let test = get_user_response("/incorrect_enum_secure", &ADMIN.to_string()).await;
    assert_eq!(StatusCode::UNAUTHORIZED, test.status());
}

async fn get_user_response(uri: &str, role: &str) -> Response {
    let app = Router::new()
        .route("/imported_enum_secure", get(imported_path_enum_secure))
        .route("/full_path_enum_secure", get(full_path_enum_secure))
        .route("/incorrect_enum_secure", get(incorrect_enum_secure))
        .layer(GrantsLayer::with_extractor(common::enum_extract));

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
