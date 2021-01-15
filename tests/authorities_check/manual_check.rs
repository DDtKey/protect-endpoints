use actix_web::dev::ServiceResponse;
use actix_web::{get, test, App, HttpResponse};

use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use actix_web::http::{header::AUTHORIZATION, StatusCode};
use actix_web_grants::authorities::{AuthDetails, AuthoritiesCheck};
use actix_web_grants::GrantsMiddleware;

const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[get("/")]
async fn different_body(details: AuthDetails) -> HttpResponse {
    if details.has_authority(ROLE_ADMIN) {
        return HttpResponse::Ok().body(ADMIN_RESPONSE);
    }
    HttpResponse::Ok().body(OTHER_RESPONSE)
}

#[get("/admin")]
async fn only_admin(details: AuthDetails) -> HttpResponse {
    if details.has_authority(ROLE_ADMIN) {
        return HttpResponse::Ok().body(ADMIN_RESPONSE);
    }
    HttpResponse::Forbidden().finish()
}

#[actix_rt::test]
async fn test_different_bodies() {
    let admin_resp = get_user_response("/", ROLE_ADMIN).await;
    let manager_resp = get_user_response("/", ROLE_MANAGER).await;

    common::test_body(admin_resp, ADMIN_RESPONSE).await;
    common::test_body(manager_resp, OTHER_RESPONSE).await;
}

#[actix_rt::test]
async fn test_forbidden() {
    let test_admin = get_user_response("/admin", ROLE_ADMIN).await;
    let test_manager = get_user_response("/admin", ROLE_MANAGER).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

async fn get_user_response(uri: &str, role: &str) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::fn_extractor(common::extract))
            .service(different_body)
            .service(only_admin),
    )
    .await;

    let req = test::TestRequest::with_header(AUTHORIZATION, role)
        .uri(uri)
        .to_request();
    test::call_service(&mut app, req).await
}
