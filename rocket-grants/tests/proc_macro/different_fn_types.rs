use crate::common::{self, ROLE_ADMIN, ROLE_MANAGER};
use rocket::http::hyper::header::AUTHORIZATION;
use rocket::http::{Header, Status};
use rocket::local::asynchronous::{Client, LocalResponse};
use rocket::serde::json::Json;
use rocket_grants::{protect, GrantsFairing};
use serde::{Deserialize, Serialize};

#[protect("ROLE_ADMIN")]
#[rocket::get("/http_response")]
async fn http_response() -> Status {
    Status::Ok
}

#[protect("ROLE_ADMIN")]
#[rocket::get("/str")]
async fn str_response() -> &'static str {
    "Hi!"
}

#[derive(Deserialize, Serialize)]
struct User {
    id: i32,
}

#[protect("ROLE_ADMIN", secure = "user_id == user.id")]
#[rocket::post("/secure/<user_id>", data = "<user>")]
async fn secure_user_id(user_id: i32, user: Json<User>) -> &'static str {
    "Hi!"
}

#[protect("ROLE_ADMIN")]
#[rocket::get("/return")]
async fn return_response() -> &'static str {
    return "Hi!";
}

#[protect("ROLE_ADMIN")]
#[rocket::get("/result?<name>")]
async fn result_response(name: Option<String>) -> Result<String, Status> {
    let name = name.as_ref().ok_or(Status::MethodNotAllowed)?;
    Ok(format!("Welcome {}!", name))
}

#[tokio::test]
async fn test_http_response() {
    let client = get_client().await;
    let test_admin = get_user_response(&client, "/http_response", ROLE_ADMIN).await;
    let test_manager = get_user_response(&client, "/http_response", ROLE_MANAGER).await;

    assert_eq!(Status::Ok, test_admin.status());
    assert_eq!(Status::Forbidden, test_manager.status());
}

#[tokio::test]
async fn test_str() {
    let client = get_client().await;
    let test_admin = get_user_response(&client, "/str", ROLE_ADMIN).await;
    let test_manager = get_user_response(&client, "/str", ROLE_MANAGER).await;

    assert_eq!(Status::Ok, test_admin.status());
    assert_eq!(Status::Forbidden, test_manager.status());

    common::test_body(test_admin, "Hi!").await;
}

#[tokio::test]
async fn test_return() {
    let client = get_client().await;
    let test_ok = get_user_response(&client, "/return", ROLE_ADMIN).await;
    assert_eq!(Status::Ok, test_ok.status());

    common::test_body(test_ok, "Hi!").await;
}

#[tokio::test]
async fn test_secure_with_user_id() {
    let user = User { id: 1 };
    let client = get_client().await;
    let test_ok = post_user_response(&client, "/secure/1", ROLE_ADMIN, &user).await;
    let test_err = post_user_response(&client, "/secure/2", ROLE_ADMIN, &user).await;

    assert_eq!(Status::Ok, test_ok.status());
    assert_eq!(Status::Forbidden, test_err.status());

    common::test_body(test_ok, "Hi!").await;
}

#[tokio::test]
async fn test_result() {
    let client = get_client().await;
    let test_ok = get_user_response(&client, "/result?name=Test", ROLE_ADMIN).await;
    let test_err = get_user_response(&client, "/result", ROLE_ADMIN).await;
    let test_forbidden = get_user_response(&client, "/result", ROLE_MANAGER).await;

    assert_eq!(Status::Ok, test_ok.status());
    assert_eq!(Status::MethodNotAllowed, test_err.status());
    assert_eq!(Status::Forbidden, test_forbidden.status());

    common::test_body(test_ok, "Welcome Test!").await;
}

#[rocket::catch(403)]
fn forbidden_catcher(_req: &rocket::Request) -> String {
    "Custom Forbidden error message".to_string()
}

#[rocket::catch(401)]
fn unauthorized_catcher() -> String {
    "Custom Unauthorized error message".to_string()
}

#[tokio::test]
async fn test_custom_error() {
    let app = rocket::build()
        .mount("/", rocket::routes![str_response])
        .attach(GrantsFairing::with_extractor_fn(|req| {
            Box::pin(common::extract(req))
        }))
        .register(
            "/",
            rocket::catchers!(forbidden_catcher, unauthorized_catcher),
        );
    let client = Client::untracked(app).await.unwrap();

    let test_ok = get_user_response(&client, "/str", ROLE_ADMIN).await;
    let test_forbidden = get_user_response(&client, "/str", ROLE_MANAGER).await;
    let test_unauthorized = client.get("/str").dispatch().await;

    assert_eq!(Status::Ok, test_ok.status());
    assert_eq!(Status::Forbidden, test_forbidden.status());
    assert_eq!(Status::Unauthorized, test_unauthorized.status());

    common::test_body(test_forbidden, "Custom Forbidden error message").await;
    common::test_body(test_unauthorized, "Custom Unauthorized error message").await;
}

async fn get_client() -> Client {
    let app = rocket::build()
        .mount(
            "/",
            rocket::routes![
                http_response,
                str_response,
                return_response,
                result_response,
                secure_user_id,
            ],
        )
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

async fn post_user_response<'a, T: Serialize>(
    client: &'a Client,
    uri: &'static str,
    role: &'static str,
    data: &'a T,
) -> LocalResponse<'a> {
    client
        .post(uri)
        .header(Header::new(AUTHORIZATION.as_str(), role))
        .body(serde_json::to_string(data).unwrap())
        .dispatch()
        .await
}
