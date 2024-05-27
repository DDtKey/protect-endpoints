use crate::common::{self, Role};
use protect_salvo::authorities::{AuthDetails, AuthoritiesCheck};
use protect_salvo::GrantsLayer;
use salvo::http::header::AUTHORIZATION;
use salvo::http::{Request, StatusCode};
use salvo::test::TestClient;
use salvo::{Depot, Router, TowerLayerCompat};

const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[salvo::handler]
async fn different_body(req: &mut Request, depot: &mut Depot) -> (StatusCode, &'static str) {
    let details: AuthDetails<Role> = req.extract();
    if details.has_authority(&Role::ADMIN) {
        return (StatusCode::OK, ADMIN_RESPONSE);
    }
    (StatusCode::OK, OTHER_RESPONSE)
}

#[salvo::handler]
async fn only_admin(req: &mut Request) -> (StatusCode, &'static str) {
    let details: AuthDetails<Role> = req.extract();
    if details.has_authority(&Role::ADMIN) {
        return (StatusCode::OK, ADMIN_RESPONSE);
    }
    (StatusCode::FORBIDDEN, "")
}

#[tokio::test]
async fn test_different_bodies() {
    let admin_resp = get_user_response("/", &Role::ADMIN.to_string()).await;
    let manager_resp = get_user_response("/", &Role::MANAGER.to_string()).await;

    common::test_body(admin_resp, ADMIN_RESPONSE).await;
    common::test_body(manager_resp, OTHER_RESPONSE).await;
}

#[tokio::test]
async fn test_forbidden() {
    let test_admin = get_user_response("/admin", &Role::ADMIN.to_string()).await;
    let test_manager = get_user_response("/admin", &Role::MANAGER.to_string()).await;

    assert_eq!(StatusCode::OK, test_admin.status());
    assert_eq!(StatusCode::FORBIDDEN, test_manager.status());
}

async fn get_user_response(uri: &str, role: &str) -> salvo::Response {
    let app = Router::with_path("/")
        .hoop(GrantsLayer::with_extractor(common::enum_extract).compat())
        .get(different_body)
        .push(Router::with_path("/admin").get(only_admin));

    TestClient::get(uri)
        .add_header(AUTHORIZATION, role, true)
        .send(&app)
        .await
}
