use crate::common;
use crate::common::Role::{self, ADMIN, MANAGER};
use protect_salvo::protect;
use protect_salvo::GrantsLayer;
use salvo::http::header::AUTHORIZATION;
use salvo::prelude::*;
use salvo::test::TestClient;
use salvo_extra::TowerLayerCompat;

// Using imported custom type (in `use` section)
#[protect("ADMIN", ty = "Role")]
#[handler]
async fn imported_path_enum_secure() -> StatusCode {
    StatusCode::OK
}

// Using a full path to a custom type (enum)
#[protect("crate::common::Role::ADMIN", ty = "crate::common::Role")]
#[handler]
async fn full_path_enum_secure() -> StatusCode {
    StatusCode::OK
}

// Incorrect endpoint security without Type specification
#[protect("ROLE_ADMIN")]
#[handler]
async fn incorrect_enum_secure() -> StatusCode {
    StatusCode::OK
}

#[tokio::test]
async fn test_http_response_for_imported_enum() {
    let test_admin = get_user_response("/imported_enum_secure", &ADMIN.to_string()).await;
    let test_manager = get_user_response("/imported_enum_secure", &MANAGER.to_string()).await;

    assert_eq!(Some(StatusCode::OK), test_admin.status_code);
    assert_eq!(Some(StatusCode::FORBIDDEN), test_manager.status_code);
}

#[tokio::test]
async fn test_http_response_for_full_path_enum() {
    let test_admin = get_user_response("/full_path_enum_secure", &ADMIN.to_string()).await;
    let test_manager = get_user_response("/full_path_enum_secure", &MANAGER.to_string()).await;

    assert_eq!(Some(StatusCode::OK), test_admin.status_code);
    assert_eq!(Some(StatusCode::FORBIDDEN), test_manager.status_code);
}

#[tokio::test]
async fn test_incorrect_http_response() {
    let test = get_user_response("/incorrect_enum_secure", &ADMIN.to_string()).await;
    assert_eq!(Some(StatusCode::UNAUTHORIZED), test.status_code);
}

async fn get_user_response(uri: &str, role: &str) -> Response {
    let app = Service::new(
        Router::with_path("/")
            .hoop(GrantsLayer::with_extractor(common::enum_extract).compat())
            .push(Router::with_path("/imported_enum_secure").get(imported_path_enum_secure))
            .push(Router::with_path("/full_path_enum_secure").get(full_path_enum_secure))
            .push(Router::with_path("/incorrect_enum_secure").get(incorrect_enum_secure)),
    );

    TestClient::get(format!("http://localhost{uri}"))
        .add_header(AUTHORIZATION, role, true)
        .send(&app)
        .await
}
