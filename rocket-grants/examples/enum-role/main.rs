use std::collections::HashSet;
use crate::role::Role::{self, Admin};
use rocket::http::Status;
use rocket::Request;
use rocket_grants::authorities::{AuthDetails, AuthoritiesCheck};
use rocket_grants::GrantsFairing;

mod role;

// `proc-macro` way require specify your type. It can be an import or a full path.
#[rocket_grants::protect(any("Admin", "role::Role::Manager"), ty = Role)]
// For the `Admin` or `Manager` - endpoint will give the HTTP status 200, otherwise - 403
#[rocket::get("/macro_secured")]
async fn macro_secured() -> Status {
    Status::Ok
}

// An example of programmable protection with custom type
#[rocket::get("/manual")]
async fn manual_secure(details: AuthDetails<Role>) -> &'static str {
    if details.has_authority(&Role::Admin) {
        return "Hello Admin!";
    }
    "Hello!"
}

#[rocket::launch]
// Sample application with grant protection based on extracting by your custom function
async fn rocket() -> _ {
    rocket::build()
        .mount("/api", rocket::routes![macro_secured, manual_secure])
        .attach(GrantsFairing::with_extractor_fn(|req| Box::pin(extract(req))))
}

// You can specify any of your own type (`PartialEq` + `Clone`) for the return type wrapped in a vector: rocket::Result<Vec<YOUR_TYPE_HERE>>
async fn extract(_req: &mut Request<'_>) -> Option<HashSet<Role>> {
    // Here is a place for your code to get user permissions/roles/authorities from a request
    // For example from a token or database

    // Stub example
    Some(HashSet::from([Role::Admin]))
}
