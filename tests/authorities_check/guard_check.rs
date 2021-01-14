use actix_web::dev::ServiceResponse;
use actix_web::{test, web, App, HttpResponse};

use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use actix_web::http::{header::AUTHORIZATION, StatusCode};
use actix_web_grants::{AuthorityGuard, GrantsMiddleware};

#[actix_rt::test]
async fn test_guard() {
    let admin_role = ROLE_ADMIN.to_string();
    let manager_role = ROLE_MANAGER.to_string();

    let test_admin = get_user_response("/admin", admin_role).await;
    let test_manager = get_user_response("/admin", manager_role).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::NOT_FOUND, test_manager.status());
}

async fn get_user_response(uri: &str, role: String) -> ServiceResponse {
    let mut app = test::init_service(
        App::new()
            .wrap(GrantsMiddleware::fn_extractor(common::extract))
            .service(
                web::resource("/admin")
                    .to(|| async { HttpResponse::Ok().finish() })
                    .guard(AuthorityGuard::new(ROLE_ADMIN.to_string())),
            ),
    )
    .await;

    let user = common::User {
        authorities: vec![role],
    };
    let json_user = serde_json::to_string(&user).unwrap();

    let req = test::TestRequest::with_header(AUTHORIZATION, json_user)
        .uri(uri)
        .to_request();
    test::call_service(&mut app, req).await
}
