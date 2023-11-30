use std::collections::HashSet;
use rocket::serde::json::Json;
use rocket::{get, post};
use rocket_grants::authorities::{AuthDetails, AuthoritiesCheck};
use rocket_grants::GrantsFairing;

const ROLE_ADMIN: &str = "ROLE_ADMIN";
const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

// An example of protection via `proc-macro`
#[rocket_grants::protect("OP_READ_ADMIN_INFO")]
#[get("/admin")]
async fn macro_secured() -> &'static str {
    ADMIN_RESPONSE
}

// An example of programmable protection
#[get("/")]
async fn manual_secure(details: AuthDetails) -> &'static str {
    if details.has_authority(ROLE_ADMIN) {
        return ADMIN_RESPONSE;
    }
    OTHER_RESPONSE
}

// An example of protection via `proc-macro` with secure attribute
#[rocket_grants::protect("ROLE_ADMIN", expr = "user_id == user.id")]
#[post("/resource/<user_id>", data = "<user>")]
async fn secure_with_params(user_id: i32, user: Json<User>) -> &'static str {
    ADMIN_RESPONSE
}

#[derive(serde::Deserialize)]
struct User {
    id: i32,
}

// Custom errors could be specified with Rocket catchers
#[rocket::catch(403)]
fn forbidden_catcher(_req: &rocket::Request) -> String {
    "Custom Forbidden error message".to_string()
}

#[rocket::catch(401)]
fn unauthorized_catcher() -> String {
    "Custom Unauthorized error message".to_string()
}

#[rocket::launch]
// Sample application with grant protection based on extracting by your custom function
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            rocket::routes![macro_secured, manual_secure, secure_with_params],
        )
        .register(
            "/",
            rocket::catchers!(unauthorized_catcher, forbidden_catcher),
        )
        .attach(GrantsFairing::with_extractor_fn(|_req| {
            Box::pin(async move {
                Some(HashSet::from([ROLE_ADMIN.to_string()])) // just a stub
            })
        }))
}
