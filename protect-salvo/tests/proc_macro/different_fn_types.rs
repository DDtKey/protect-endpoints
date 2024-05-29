use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use protect_salvo::{protect, GrantsLayer};
use salvo::http::header::{AUTHORIZATION, CONTENT_TYPE};
use salvo::prelude::*;
use salvo::test::TestClient;
use serde::{Deserialize, Serialize};

#[protect("ROLE_ADMIN")]
#[handler]
async fn http_response(resp: &mut Response) {
    resp.status_code(StatusCode::OK);
}

#[protect("ROLE_ADMIN")]
#[handler]
async fn str_response() -> &'static str {
    "Hi!"
}

#[derive(Deserialize, Serialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
struct User {
    id: i32,
}

#[derive(Deserialize, Serialize, Extractible)]
#[salvo(extract(default_source(from = "param")))]
struct UserParam {
    user_id: i32,
}

#[protect("ROLE_ADMIN", expr = "user_param.user_id == user.id")]
#[handler]
async fn secure_user_id(user_param: UserParam, user: User) -> &'static str {
    "Hi!"
}

#[protect("ROLE_ADMIN")]
#[handler]
async fn return_response() -> &'static str {
    return "Hi!";
}

#[protect("ROLE_ADMIN")]
#[handler]
async fn result_response(payload: common::NamePayload) -> Result<String, StatusCode> {
    let name = payload.name.as_ref().ok_or(StatusCode::BAD_REQUEST)?;
    Ok(format!("Welcome {}!", name))
}

#[tokio::test]
async fn test_http_response() {
    let test_admin = get_user_response("/http_response", ROLE_ADMIN).await;
    let test_manager = get_user_response("/http_response", ROLE_MANAGER).await;

    assert_eq!(Some(StatusCode::OK), test_admin.status_code);
    assert_eq!(Some(StatusCode::FORBIDDEN), test_manager.status_code);
}

#[tokio::test]
async fn test_str() {
    let test_admin = get_user_response("/str", ROLE_ADMIN).await;
    let test_manager = get_user_response("/str", ROLE_MANAGER).await;

    assert_eq!(Some(StatusCode::OK), test_admin.status_code);
    assert_eq!(Some(StatusCode::FORBIDDEN), test_manager.status_code);

    common::test_body(test_admin, "Hi!").await;
}

#[tokio::test]
async fn test_return() {
    let test_ok = get_user_response("/return", ROLE_ADMIN).await;
    assert_eq!(Some(StatusCode::OK), test_ok.status_code);

    common::test_body(test_ok, "Hi!").await;
}

#[tokio::test]
async fn test_secure_with_user_id() {
    let user = User { id: 1 };
    let test_ok = post_user_response("/secure/1", ROLE_ADMIN, &user).await;
    let test_forbidden = post_user_response("/secure/2", ROLE_ADMIN, &user).await;

    assert_eq!(Some(StatusCode::OK), test_ok.status_code);
    assert_eq!(Some(StatusCode::FORBIDDEN), test_forbidden.status_code);

    common::test_body(test_ok, "Hi!").await;
}

#[tokio::test]
async fn test_result() {
    let test_ok = get_user_response("/result?name=Test", ROLE_ADMIN).await;
    let test_err = get_user_response("/result", ROLE_ADMIN).await;
    let test_forbidden = get_user_response("/result", ROLE_MANAGER).await;

    assert_eq!(Some(StatusCode::OK), test_ok.status_code);
    assert_eq!(Some(StatusCode::BAD_REQUEST), test_err.status_code);
    assert_eq!(Some(StatusCode::FORBIDDEN), test_forbidden.status_code);

    common::test_body(test_ok, "Welcome Test!").await;
}

async fn get_user_response(uri: &str, role: &str) -> Response {
    let app = Service::new(
        Router::with_path("/")
            .hoop(GrantsLayer::with_extractor(common::extract).compat())
            .push(Router::with_path("/http_response").get(http_response))
            .push(Router::with_path("/str").get(str_response))
            .push(Router::with_path("/return").get(return_response))
            .push(Router::with_path("/result").get(result_response)),
    );

    TestClient::get(format!("http://localhost{uri}"))
        .add_header(AUTHORIZATION, role, true)
        .send(&app)
        .await
}

async fn post_user_response<T: Serialize>(uri: &str, role: &str, data: &T) -> Response {
    let app = Service::new(
        Router::with_path("/")
            .hoop(GrantsLayer::with_extractor(common::extract).compat())
            .push(Router::with_path("/secure/<user_id>").post(secure_user_id)),
    );

    TestClient::post(format!("http://localhost{uri}"))
        .add_header(AUTHORIZATION, role, true)
        .add_header(CONTENT_TYPE, "application/json", true)
        .body(serde_json::to_string(data).unwrap())
        .send(&app)
        .await
}
