use crate::common::{self, Role};
use rocket::http::hyper::header::AUTHORIZATION;
use rocket::http::{Header, Status};
use rocket::local::asynchronous::Client;
use rocket::local::asynchronous::LocalResponse;
use rocket_grants::permissions::{AuthDetails, PermissionsCheck};
use rocket_grants::GrantsFairing;

const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[rocket::get("/")]
async fn different_body(details: AuthDetails<Role>) -> &'static str {
    if details.has_permission(&Role::Admin) {
        return ADMIN_RESPONSE;
    }
    OTHER_RESPONSE
}

#[rocket::get("/admin")]
async fn only_admin(details: AuthDetails<Role>) -> Result<&'static str, Status> {
    if details.has_permission(&Role::Admin) {
        return Ok(ADMIN_RESPONSE);
    }
    Err(Status::Forbidden)
}

#[tokio::test]
async fn test_different_bodies() {
    let client = get_client().await;
    let admin_resp = get_user_response(&client, "/", Role::Admin.to_string()).await;
    let manager_resp = get_user_response(&client, "/", Role::Manager.to_string()).await;

    common::test_body(admin_resp, ADMIN_RESPONSE).await;
    common::test_body(manager_resp, OTHER_RESPONSE).await;
}

#[tokio::test]
async fn test_forbidden() {
    let client = get_client().await;
    let test_admin = get_user_response(&client, "/admin", Role::Admin.to_string()).await;
    let test_manager = get_user_response(&client, "/admin", Role::Manager.to_string()).await;

    assert_eq!(Status::Ok, test_admin.status());
    assert_eq!(Status::Forbidden, test_manager.status());
}

async fn get_client() -> Client {
    let app = rocket::build()
        .mount("/", rocket::routes![different_body, only_admin])
        .attach(GrantsFairing::with_extractor_fn(|req| {
            Box::pin(common::enum_extract(req))
        }));
    Client::untracked(app).await.unwrap()
}

async fn get_user_response<'a>(
    client: &'a Client,
    uri: &'a str,
    role: String,
) -> LocalResponse<'a> {
    client
        .get(uri)
        .header(Header::new(AUTHORIZATION.as_str(), role))
        .dispatch()
        .await
}
