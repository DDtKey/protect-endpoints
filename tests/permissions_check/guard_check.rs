use actix_web::dev::ServiceResponse;
use actix_web::{test, web, App, HttpResponse};

use crate::common::{self, Role, ROLE_ADMIN, ROLE_MANAGER};
use actix_web::http::{header::AUTHORIZATION, StatusCode};
use actix_web_grants::{GrantsMiddleware, PermissionGuard};

#[actix_rt::test]
async fn test_guard() {
    let test_admin = get_user_response("/admin", ROLE_ADMIN).await;
    let test_manager = get_user_response("/admin", ROLE_MANAGER).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::NOT_FOUND, test_manager.status());
}

#[actix_rt::test]
async fn test_enum_guard() {
    let test_admin = get_user_response_with_enum("/admin", &Role::ADMIN.to_string()).await;
    let test_manager = get_user_response_with_enum("/admin", &Role::MANAGER.to_string()).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::NOT_FOUND, test_manager.status());
}

async fn get_user_response(uri: &str, role: &str) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::with_extractor(common::extract))
            .service(
                web::resource("/admin")
                    .to(|| async { HttpResponse::Ok().finish() })
                    .guard(PermissionGuard::new(ROLE_ADMIN.to_string())),
            ),
    )
    .await;

    let req = test::TestRequest::with_header(AUTHORIZATION, role)
        .uri(uri)
        .to_request();
    test::call_service(&mut app, req).await
}

async fn get_user_response_with_enum(uri: &str, role: &str) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::with_extractor(common::enum_extract))
            .service(
                web::resource("/admin")
                    .to(|| async { HttpResponse::Ok().finish() })
                    .guard(PermissionGuard::new(Role::ADMIN)),
            ),
    )
    .await;

    let req = test::TestRequest::with_header(AUTHORIZATION, role)
        .uri(uri)
        .to_request();
    test::call_service(&mut app, req).await
}
