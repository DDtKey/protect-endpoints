use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use axum::body::Body;
use axum::extract::{Path, Query};
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{Request, StatusCode};
use axum::response::Response;
use axum::routing::{get, post};
use axum::{Json, Router};
use protect_axum::{protect, GrantsLayer};
use serde::{Deserialize, Serialize};
use tower::ServiceExt;

#[protect("ROLE_ADMIN")]
async fn http_response() -> Response {
    Response::builder()
        .status(StatusCode::OK)
        .body(().into())
        .unwrap()
}

#[protect("ROLE_ADMIN")]
async fn str_response() -> &'static str {
    "Hi!"
}

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
}

#[protect("ROLE_ADMIN", expr = "user_id == user.id")]
async fn secure_user_id(Path(user_id): Path<i32>, Json(user): Json<User>) -> &'static str {
    "Hi!"
}

#[protect("ROLE_ADMIN")]
async fn return_response() -> &'static str {
    return "Hi!";
}

#[protect("ROLE_ADMIN")]
async fn result_response(
    payload: Query<common::NamePayload>,
) -> Result<String, (StatusCode, &'static str)> {
    let name = payload
        .0
        .name
        .as_ref()
        .ok_or((StatusCode::BAD_REQUEST, "bad request"))?;
    Ok(format!("Welcome {}!", name))
}

#[tokio::test]
async fn test_http_response() {
    let test_admin = get_user_response("/http_response", ROLE_ADMIN).await;
    let test_manager = get_user_response("/http_response", ROLE_MANAGER).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

#[tokio::test]
async fn test_str() {
    let test_admin = get_user_response("/str", ROLE_ADMIN).await;
    let test_manager = get_user_response("/str", ROLE_MANAGER).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());

    common::test_body(test_admin, "Hi!").await;
}

#[tokio::test]
async fn test_return() {
    let test_ok = get_user_response("/return", ROLE_ADMIN).await;
    assert_eq!(StatusCode::OK, test_ok.status());

    common::test_body(test_ok, "Hi!").await;
}

#[tokio::test]
async fn test_secure_with_user_id() {
    let user = User { id: 1 };
    let test_ok = post_user_response("/secure/1", ROLE_ADMIN, &user).await;
    let test_forbidden = post_user_response("/secure/2", ROLE_ADMIN, &user).await;

    assert_eq!(StatusCode::OK, test_ok.status());
    assert_eq!(StatusCode::FORBIDDEN, test_forbidden.status());

    common::test_body(test_ok, "Hi!").await;
}

#[tokio::test]
async fn test_result() {
    let test_ok = get_user_response("/result?name=Test", ROLE_ADMIN).await;
    let test_err = get_user_response("/result", ROLE_ADMIN).await;
    let test_forbidden = get_user_response("/result", ROLE_MANAGER).await;

    assert_eq!(StatusCode::OK, test_ok.status());
    assert_eq!(StatusCode::BAD_REQUEST, test_err.status());
    assert_eq!(StatusCode::FORBIDDEN, test_forbidden.status());

    common::test_body(test_ok, "Welcome Test!").await;
    common::test_body(test_err, "bad request").await;
}

async fn get_user_response(uri: &str, role: &str) -> Response {
    let app = Router::new()
        .route("/http_response", get(http_response))
        .route("/str", get(str_response))
        .route("/return", get(return_response))
        .route("/result", get(result_response))
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

async fn post_user_response<T: Serialize>(uri: &str, role: &str, data: &T) -> Response {
    let app = Router::new()
        .route("/secure/{user_id}", post(secure_user_id))
        .layer(GrantsLayer::with_extractor(common::extract));

    app.oneshot(
        Request::builder()
            .method("POST")
            .uri(uri)
            .header(AUTHORIZATION, role)
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(data).unwrap())
            .unwrap(),
    )
    .await
    .unwrap()
}
