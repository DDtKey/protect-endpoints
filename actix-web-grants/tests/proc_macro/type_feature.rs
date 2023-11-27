use crate::common;
use crate::common::Permission;
use crate::common::Role::{self, ADMIN, MANAGER};
use actix_web::dev::ServiceResponse;
use actix_web::http::header::AUTHORIZATION;
use actix_web::http::StatusCode;
use actix_web::{get, test, App, HttpResponse};
use actix_web_grants::{protect, GrantsMiddleware};

// Using imported custom type (in `use` section)
#[get("/imported_enum_secure")]
#[protect("ADMIN", ty = "Role")]
async fn imported_path_enum_secure() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Using a full path to a custom type (enum)
#[get("/full_path_enum_secure")]
#[protect("crate::common::Role::ADMIN", ty = "crate::common::Role")]
async fn full_path_enum_secure() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Incorrect endpoint security without Type specification
#[get("/incorrect_enum_secure")]
#[protect("ROLE_ADMIN")]
async fn incorrect_enum_secure() -> HttpResponse {
    HttpResponse::Ok().finish()
}

// Combine different type of Role & Permissions
#[get("/role_and_permission_enums_secure")]
#[protect("ADMIN", ty = "Role")]
#[protect("Permission::WRITE", ty = "Permission")]
async fn role_and_permission_enums_secure() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[actix_rt::test]
async fn test_http_response_for_imported_enum() {
    let test_admin = get_user_response("/imported_enum_secure", &ADMIN.to_string()).await;
    let test_manager = get_user_response("/imported_enum_secure", &MANAGER.to_string()).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

#[actix_rt::test]
async fn test_http_response_for_role_and_permission_enums() {
    let test_admin_with_write =
        get_user_response(
            "/role_and_permission_enums_secure",
            &format!("{ADMIN},WRITE"),
        )
        .await;
    // there is no `write` permission
    let test_admin =
        get_user_response("/role_and_permission_enums_secure", &ADMIN.to_string()).await;

    assert_eq!(StatusCode::OK, test_admin_with_write.status());
    assert_eq!(StatusCode::FORBIDDEN, test_admin.status());
}

#[actix_rt::test]
async fn test_http_response_for_full_path_enum() {
    let test_admin = get_user_response("/full_path_enum_secure", &ADMIN.to_string()).await;
    let test_manager = get_user_response("/full_path_enum_secure", &MANAGER.to_string()).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

#[actix_rt::test]
async fn test_incorrect_http_response() {
    let test = get_user_response("/incorrect_enum_secure", &ADMIN.to_string()).await;

    assert_eq!(StatusCode::UNAUTHORIZED, test.status());
}

async fn get_user_response(uri: &str, role: &str) -> ServiceResponse {
    let app =
        test::init_service(
            App::new()
                .wrap(GrantsMiddleware::with_extractor(common::enum_extract::<Role>))
                .wrap(GrantsMiddleware::with_extractor(common::enum_extract::<Permission>))
                .service(imported_path_enum_secure)
                .service(full_path_enum_secure)
                .service(incorrect_enum_secure)
                .service(role_and_permission_enums_secure),
        )
        .await;

    let req = test::TestRequest::default()
        .insert_header((AUTHORIZATION, role))
        .uri(uri)
        .to_request();
    test::call_service(&app, req).await
}
