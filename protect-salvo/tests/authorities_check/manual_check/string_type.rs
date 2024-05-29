use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use salvo::http::header::AUTHORIZATION;
use salvo::http::StatusCode;
use salvo::test::TestClient;
use salvo::{Response, Router, Service, TowerLayerCompat, Writer};

use protect_endpoints_core::authorities::AuthoritiesCheck;
use protect_salvo::authorities::AuthDetails;
use protect_salvo::GrantsLayer;

const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[salvo::handler]
async fn different_body(details: AuthDetails, res: &mut Response) {
    if details.has_authority(ROLE_ADMIN) {
        res.stuff(StatusCode::OK, ADMIN_RESPONSE);
        return;
    }
    res.stuff(StatusCode::OK, OTHER_RESPONSE);
}

#[salvo::handler]
async fn only_admin(details: AuthDetails, res: &mut Response) {
    if details.has_authority(ROLE_ADMIN) {
        res.stuff(StatusCode::OK, ADMIN_RESPONSE);
        return;
    }
    res.stuff(StatusCode::FORBIDDEN, OTHER_RESPONSE);
}

#[tokio::test]
async fn test_different_bodies() {
    let admin_resp = get_user_response("/", ROLE_ADMIN).await;
    let manager_resp = get_user_response("/", ROLE_MANAGER).await;

    common::test_body(admin_resp, ADMIN_RESPONSE).await;
    common::test_body(manager_resp, OTHER_RESPONSE).await;
}

#[tokio::test]
async fn test_forbidden() {
    let test_admin = get_user_response("/admin", ROLE_ADMIN).await;
    let test_manager = get_user_response("/admin", ROLE_MANAGER).await;

    assert_eq!(Some(StatusCode::OK), test_admin.status_code);
    assert_eq!(Some(StatusCode::FORBIDDEN), test_manager.status_code);
}

async fn get_user_response(uri: &str, role: &str) -> Response {
    let app = Service::new(
        Router::with_path("/")
            .hoop(GrantsLayer::with_extractor(common::extract).compat())
            .get(different_body)
            .push(Router::with_path("/admin").get(only_admin)),
    );

    TestClient::get(format!("http://localhost{uri}"))
        .add_header(AUTHORIZATION, role, true)
        .send(&app)
        .await
}
