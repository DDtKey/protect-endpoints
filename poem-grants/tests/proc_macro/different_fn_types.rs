use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use poem::error::MethodNotAllowedError;
use poem::http::header::AUTHORIZATION;
use poem::http::StatusCode;
use poem::test::{TestClient, TestResponse};
use poem::web::{Json, Path, Query};
use poem::{EndpointExt, Response, Route};
use poem_grants::{protect, GrantsMiddleware};
use serde::{Deserialize, Serialize};

#[protect("ROLE_ADMIN")]
#[poem::handler]
async fn http_response() -> Response {
    Response::builder().status(StatusCode::OK).finish()
}

#[protect("ROLE_ADMIN")]
#[poem::handler]
async fn str_response() -> &'static str {
    "Hi!"
}

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
}

#[protect("ROLE_ADMIN", expr = "*user_id == user.id")]
#[poem::handler]
async fn secure_user_id(user_id: Path<i32>, user: Json<User>) -> &'static str {
    "Hi!"
}

#[protect("ROLE_ADMIN")]
#[poem::handler]
async fn return_response() -> &'static str {
    return "Hi!";
}

#[protect("ROLE_ADMIN")]
#[poem::handler]
async fn result_response(
    payload: Query<common::NamePayload>,
) -> poem::Result<String, MethodNotAllowedError> {
    let name = payload.name.as_ref().ok_or(MethodNotAllowedError)?;
    Ok(format!("Welcome {}!", name))
}

#[protect("ROLE_ADMIN")]
#[poem::handler]
fn sync_handler() -> Response {
    Response::builder().status(StatusCode::OK).finish()
}

#[tokio::test]
async fn test_http_response() {
    let test_admin = get_user_response("/http_response", ROLE_ADMIN).await;
    let test_manager = get_user_response("/http_response", ROLE_MANAGER).await;

    test_admin.assert_status_is_ok();
    test_manager.assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_str() {
    let test_admin = get_user_response("/str", ROLE_ADMIN).await;
    let test_manager = get_user_response("/str", ROLE_MANAGER).await;

    test_admin.assert_status_is_ok();
    test_manager.assert_status(StatusCode::FORBIDDEN);

    common::test_body(test_admin, "Hi!").await;
    common::test_body(test_manager, "Forbidden request").await;
}

#[tokio::test]
async fn test_return() {
    let test_ok = get_user_response("/return", ROLE_ADMIN).await;
    test_ok.assert_status_is_ok();

    common::test_body(test_ok, "Hi!").await;
}

#[tokio::test]
async fn test_secure_with_user_id() {
    let user = User { id: 1 };
    let test_ok = post_user_response("/secure/1", ROLE_ADMIN, &user).await;
    let test_err = post_user_response("/secure/2", ROLE_ADMIN, &user).await;

    test_ok.assert_status_is_ok();
    test_err.assert_status(StatusCode::FORBIDDEN);

    common::test_body(test_ok, "Hi!").await;
    common::test_body(test_err, "Forbidden request").await;
}

#[tokio::test]
async fn test_result() {
    let test_ok = get_user_response("/result?name=Test", ROLE_ADMIN).await;
    let test_err = get_user_response("/result", ROLE_ADMIN).await;
    let test_forbidden = get_user_response("/result", ROLE_MANAGER).await;

    test_ok.assert_status_is_ok();
    test_err.assert_status(StatusCode::METHOD_NOT_ALLOWED);
    test_forbidden.assert_status(StatusCode::FORBIDDEN);

    common::test_body(test_ok, "Welcome Test!").await;
    common::test_body(test_err, "method not allowed").await;
    common::test_body(test_forbidden, "Forbidden request").await;
}

#[tokio::test]
async fn test_sync_handler() {
    let test_admin = get_user_response("/sync_handler", ROLE_ADMIN).await;
    let test_manager = get_user_response("/sync_handler", ROLE_MANAGER).await;

    test_admin.assert_status_is_ok();
    test_manager.assert_status(StatusCode::FORBIDDEN);
}

async fn get_user_response(uri: &str, role: &str) -> TestResponse {
    let app = Route::new()
        .at("/http_response", http_response)
        .at("/str", str_response)
        .at("/return", return_response)
        .at("/result", result_response)
        .with(GrantsMiddleware::with_extractor(common::extract));
    let cli = TestClient::new(app);

    cli.get(uri).header(AUTHORIZATION, role).send().await
}

async fn post_user_response<T: Serialize>(uri: &str, role: &str, data: &T) -> TestResponse {
    let app = Route::new()
        .at("/secure/:user_id", secure_user_id)
        .with(GrantsMiddleware::with_extractor(common::extract));
    let cli = TestClient::new(app);

    cli.post(uri)
        .header(AUTHORIZATION, role)
        .body_json(data)
        .send()
        .await
}
