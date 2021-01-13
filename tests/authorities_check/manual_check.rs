use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{get, test, App, Error, HttpResponse};

use crate::common::*;
use actix_web::error::ErrorUnauthorized;
use actix_web::http::{header::AUTHORIZATION, HeaderValue, StatusCode};
use actix_web_grants::authorities::{AuthDetails, AuthoritiesCheck};
use actix_web_grants::GrantsMiddleware;
use futures::join;
use std::sync::Arc;

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
    let admin_role = ROLE_ADMIN.to_string();
    let manager_role = ROLE_MANAGER.to_string();

    let test_admin = test_body_for_role("/", admin_role, ADMIN_RESPONSE);
    let test_manager = test_body_for_role("/", manager_role, OTHER_RESPONSE);
    join!(test_admin, test_manager);
}

#[actix_rt::test]
async fn test_forbidden() {
    let admin_role = ROLE_ADMIN.to_string();
    let manager_role = ROLE_MANAGER.to_string();

    let test_admin = get_user_response("/admin", admin_role).await;
    let test_manager = get_user_response("/admin", manager_role).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

async fn test_body_for_role(uri: &str, role: String, expected_body: &str) {
    let resp = get_user_response(uri, role).await;
    let body = test::read_body(resp).await;

    assert_eq!(expected_body, String::from_utf8(body.to_vec()).unwrap());
}

async fn get_user_response(uri: &str, role: String) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::fn_extractor(extract))
            .service(different_body)
            .service(only_admin),
    )
    .await;

    let user: User = User {
        authorities: vec![role],
    };
    let json_user = serde_json::to_string(&user).unwrap();

    let req = test::TestRequest::with_header(AUTHORIZATION, json_user)
        .uri(uri)
        .to_request();
    test::call_service(&mut app, req).await
}

async fn extract(req: Arc<ServiceRequest>) -> Result<Vec<String>, Error> {
    let auth_header: Option<&str> = req
        .headers()
        .get(AUTHORIZATION)
        .map(HeaderValue::to_str)
        .filter(Result::is_ok)
        .map(Result::unwrap);

    let authorities = auth_header
        .map(serde_json::from_str::<User>)
        .filter(|result| result.is_ok())
        .map(|result| result.unwrap().authorities)
        .ok_or_else(|| ErrorUnauthorized("Authorization header incorrect!"))?;

    Ok(authorities)
}
