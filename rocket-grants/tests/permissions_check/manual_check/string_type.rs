use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use rocket::http::hyper::header::AUTHORIZATION;
use rocket::http::{Header, Status};
use rocket::local::asynchronous::{Client, LocalResponse};
use rocket_grants::permissions::{AuthDetails, PermissionsCheck};
use rocket_grants::GrantsFairing;

const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[rocket::get("/")]
async fn different_body(details: AuthDetails) -> &'static str {
    if details.has_permission(ROLE_ADMIN) {
        return ADMIN_RESPONSE;
    }
    OTHER_RESPONSE
}

#[rocket::get("/admin")]
async fn only_admin(details: AuthDetails) -> Result<&'static str, Status> {
    if details.has_permission(ROLE_ADMIN) {
        return Ok(ADMIN_RESPONSE);
    }
    Err(Status::Forbidden)
}

#[tokio::test]
async fn test_different_bodies() {
    let client = get_client().await;
    let admin_resp = get_user_response(&client, "/", ROLE_ADMIN).await;
    let manager_resp = get_user_response(&client, "/", ROLE_MANAGER).await;

    common::test_body(admin_resp, ADMIN_RESPONSE).await;
    common::test_body(manager_resp, OTHER_RESPONSE).await;
}

#[tokio::test]
async fn test_forbidden() {
    let client = get_client().await;
    let test_admin = get_user_response(&client, "/admin", ROLE_ADMIN).await;
    let test_manager = get_user_response(&client, "/admin", ROLE_MANAGER).await;

    assert_eq!(Status::Ok, test_admin.status());
    assert_eq!(Status::Forbidden, test_manager.status());
}

async fn get_client() -> Client {
    let app = rocket::build()
        .mount("/", rocket::routes![different_body, only_admin])
        .attach(GrantsFairing::with_extractor_fn(|req| {
            Box::pin(common::extract(req))
        }));
    Client::untracked(app).await.unwrap()
}
async fn get_user_response<'a>(
    client: &'a Client,
    uri: &'static str,
    role: &'static str,
) -> LocalResponse<'a> {
    client
        .get(uri)
        .header(Header::new(AUTHORIZATION.as_str(), role))
        .dispatch()
        .await
}
