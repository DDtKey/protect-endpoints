use rocket::serde::json::Json;
use rocket::{get, post};
use rocket_grants::permissions::{AuthDetails, PermissionsCheck};
use rocket_grants::GrantsFairing;

const ROLE_ADMIN: &str = "ROLE_ADMIN";
const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

// An example of protection via `proc-macro`
#[rocket_grants::has_permissions("OP_READ_ADMIN_INFO")]
#[get("/admin")]
async fn macro_secured() -> &'static str {
    ADMIN_RESPONSE
}

// An example of programmable protection
#[get("/")]
async fn manual_secure(details: AuthDetails) -> &'static str {
    if details.has_permission(ROLE_ADMIN) {
        return ADMIN_RESPONSE;
    }
    OTHER_RESPONSE
}

// An example of protection via `proc-macro` with secure attribute
#[rocket_grants::has_permissions("ROLE_ADMIN", secure = "user_id == user.id")]
#[post("/resource/<user_id>", data = "<user>")]
async fn secure_with_params(user_id: i32, user: Json<User>) -> &'static str {
    ADMIN_RESPONSE
}

#[derive(serde::Deserialize)]
struct User {
    id: i32,
}

#[rocket::launch]
// Sample application with grant protection based on extracting by your custom function
async fn rocket() -> _ {
    rocket::build()
        .mount(
            "/api",
            rocket::routes![macro_secured, manual_secure, secure_with_params],
        )
        .attach(GrantsFairing::with_extractor_fn(|_req| {
            Box::pin(async move {
                Some(vec![ROLE_ADMIN.to_string()]) // just a stub
            })
        }))
}
