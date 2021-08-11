use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use actix_web::dev::ServiceResponse;
use actix_web::error::ErrorBadRequest;
use actix_web::http::{header::AUTHORIZATION, StatusCode};
use actix_web::{get, http, post, test, web, App, Error, HttpResponse};
use actix_web_grants::{proc_macro::has_roles, GrantsMiddleware};
use serde::{Deserialize, Serialize};

#[get("/http_response")]
#[has_roles("ADMIN")]
async fn http_response() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/str")]
#[has_roles("ADMIN")]
async fn str_response() -> &'static str {
    "Hi!"
}

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
}

#[post("/secure/{user_id}")]
#[has_roles("ADMIN", secure = "user_id==user.id")]
async fn secure_user_id(web::Path(user_id): web::Path<i32>, user: web::Json<User>) -> &'static str {
    "Hi!"
}

#[get("/return")]
#[has_roles("ADMIN")]
async fn return_response() -> &'static str {
    return "Hi!";
}

#[get("/result")]
#[has_roles("ADMIN")]
async fn result_response(payload: web::Query<common::NamePayload>) -> Result<String, Error> {
    let common::NamePayload { name } = payload.0;
    let name = name.ok_or(ErrorBadRequest("Query param not found!"))?;
    Ok(format!("Welcome {}!", name))
}

#[actix_rt::test]
async fn test_http_response() {
    let test_admin = get_user_response("/http_response", ROLE_ADMIN).await;
    let test_manager = get_user_response("/http_response", ROLE_MANAGER).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

#[actix_rt::test]
async fn test_str() {
    let test_admin = get_user_response("/str", ROLE_ADMIN).await;
    let test_manager = get_user_response("/str", ROLE_MANAGER).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());

    common::test_body(test_admin, "Hi!").await;
    common::test_body(test_manager, "").await;
}

#[actix_rt::test]
async fn test_return() {
    let test_ok = get_user_response("/return", ROLE_ADMIN).await;
    assert_eq!(StatusCode::OK, test_ok.status());

    common::test_body(test_ok, "Hi!").await;
}

#[actix_rt::test]
async fn test_secure_with_user_id() {
    let user = User { id: 1 };
    let test_ok = post_user_response("/secure/1", ROLE_ADMIN, &user).await;
    let test_err = post_user_response("/secure/2", ROLE_ADMIN, &user).await;

    assert_eq!(StatusCode::OK, test_ok.status());
    assert_eq!(StatusCode::FORBIDDEN, test_err.status());

    common::test_body(test_ok, "Hi!").await;
    common::test_body(test_err, "").await;
}

#[actix_rt::test]
async fn test_result() {
    let test_ok = get_user_response("/result?name=Test", ROLE_ADMIN).await;
    let test_err = get_user_response("/result", ROLE_ADMIN).await;

    assert_eq!(StatusCode::OK, test_ok.status());
    assert_eq!(StatusCode::BAD_REQUEST, test_err.status());

    common::test_body(test_ok, "Welcome Test!").await;
    common::test_body(test_err, "Query param not found!").await;
}

async fn get_user_response(uri: &str, role: &str) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::with_extractor(common::extract))
            .service(http_response)
            .service(str_response)
            .service(return_response)
            .service(result_response),
    )
    .await;

    let req = test::TestRequest::with_header(AUTHORIZATION, role)
        .uri(uri)
        .to_request();
    test::call_service(&mut app, req).await
}

async fn post_user_response<T: Serialize>(uri: &str, role: &str, data: &T) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::with_extractor(common::extract))
            .service(secure_user_id),
    )
    .await;
    let req = test::TestRequest::with_header(AUTHORIZATION, role)
        .uri(uri)
        .set_json(data)
        .method(http::Method::POST)
        .to_request();
    test::call_service(&mut app, req).await
}
