use crate::common::{
    self,
    Role::{self, ADMIN, MANAGER},
};
use rocket::http::hyper::header::AUTHORIZATION;
use rocket::http::{Header, Status};
use rocket::local::asynchronous::{Client, LocalResponse};
use rocket_grants::{has_roles, GrantsFairing};

// Using imported custom type (in `use` section)
#[has_roles("ADMIN", type = "Role")]
#[rocket::get("/imported_enum_secure")]
async fn imported_path_enum_secure() -> Status {
    Status::Ok
}

// Using a full path to a custom type (enum)
#[has_roles("crate::common::Role::ADMIN", type = "crate::common::Role")]
#[rocket::get("/full_path_enum_secure")]
async fn full_path_enum_secure() -> Status {
    Status::Ok
}

// Incorrect endpoint security without Type specification
#[has_roles("ADMIN")]
#[rocket::get("/incorrect_enum_secure")]
async fn incorrect_enum_secure() -> Status {
    Status::Ok
}

#[tokio::test]
async fn test_http_response_for_imported_enum() {
    let client = get_client().await;
    let test_admin = get_user_response(&client, "/imported_enum_secure", ADMIN.to_string()).await;
    let test_manager =
        get_user_response(&client, "/imported_enum_secure", MANAGER.to_string()).await;

    assert_eq!(Status::Ok, test_admin.status());
    assert_eq!(Status::Forbidden, test_manager.status());
}

#[tokio::test]
async fn test_http_response_for_full_path_enum() {
    let client = get_client().await;
    let test_admin = get_user_response(&client, "/full_path_enum_secure", ADMIN.to_string()).await;
    let test_manager =
        get_user_response(&client, "/full_path_enum_secure", MANAGER.to_string()).await;

    assert_eq!(Status::Ok, test_admin.status());
    assert_eq!(Status::Forbidden, test_manager.status());
}

#[tokio::test]
async fn test_incorrect_http_response() {
    let client = get_client().await;
    let test = get_user_response(&client, "/incorrect_enum_secure", ADMIN.to_string()).await;
    assert_eq!(Status::Unauthorized, test.status());
}

async fn get_client() -> Client {
    let app = rocket::build()
        .mount(
            "/",
            rocket::routes![
                imported_path_enum_secure,
                full_path_enum_secure,
                incorrect_enum_secure,
            ],
        )
        .attach(GrantsFairing::with_extractor_fn(|req| {
            Box::pin(common::enum_extract(req))
        }));
    Client::untracked(app).await.unwrap()
}
async fn get_user_response<'a>(
    client: &'a Client,
    uri: &'static str,
    role: String,
) -> LocalResponse<'a> {
    client
        .get(uri)
        .header(Header::new(AUTHORIZATION.as_str(), role))
        .dispatch()
        .await
}
