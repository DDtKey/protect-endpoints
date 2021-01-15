use actix_web::dev::ServiceResponse;
use actix_web::{get, test, App, HttpResponse};

use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use actix_web::http::{header::AUTHORIZATION, StatusCode};
use actix_web_grants::{proc_macro::has_roles, GrantsMiddleware};

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

async fn get_user_response(uri: &str, role: &str) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::fn_extractor(common::extract))
            .service(str_response)
            .service(http_response),
    )
    .await;

    let req = test::TestRequest::with_header(AUTHORIZATION, role)
        .uri(uri)
        .to_request();
    test::call_service(&mut app, req).await
}
