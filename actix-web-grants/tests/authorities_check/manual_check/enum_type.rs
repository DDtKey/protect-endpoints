use actix_web::body::{BoxBody, EitherBody};
use actix_web::dev::ServiceResponse;
use actix_web::{get, test, App, HttpResponse};

use crate::common::{self, Role};
use actix_web::http::{header::AUTHORIZATION, StatusCode};
use actix_web_grants::authorities::{AuthDetails, AuthoritiesCheck};
use actix_web_grants::GrantsMiddleware;

const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[get("/")]
async fn different_body(details: AuthDetails<Role>) -> HttpResponse {
    if details.has_authority(&Role::ADMIN) {
        return HttpResponse::Ok().body(ADMIN_RESPONSE);
    }
    HttpResponse::Ok().body(OTHER_RESPONSE)
}

#[get("/admin")]
async fn only_admin(details: AuthDetails<Role>) -> HttpResponse {
    if details.has_authority(&Role::ADMIN) {
        return HttpResponse::Ok().body(ADMIN_RESPONSE);
    }
    HttpResponse::Forbidden().finish()
}

#[actix_rt::test]
async fn test_different_bodies() {
    let admin_resp = get_user_response("/", &Role::ADMIN.to_string()).await;
    let manager_resp = get_user_response("/", &Role::MANAGER.to_string()).await;

    common::test_body(admin_resp, ADMIN_RESPONSE).await;
    common::test_body(manager_resp, OTHER_RESPONSE).await;
}

#[actix_rt::test]
async fn test_forbidden() {
    let test_admin = get_user_response("/admin", &Role::ADMIN.to_string()).await;
    let test_manager = get_user_response("/admin", &Role::MANAGER.to_string()).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

async fn get_user_response(uri: &str, role: &str) -> ServiceResponse<EitherBody<BoxBody>> {
    let app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::with_extractor(
                common::enum_extract::<Role>,
            ))
            .service(different_body)
            .service(only_admin),
    )
    .await;

    let req = test::TestRequest::default()
        .insert_header((AUTHORIZATION, role))
        .uri(uri)
        .to_request();
    test::call_service(&app, req).await
}
