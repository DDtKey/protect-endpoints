use crate::role::Role::{self, Admin};
use actix_web::dev::ServiceRequest;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};
use actix_web_grants::permissions::{AuthDetails, RolesCheck};
use actix_web_grants::{proc_macro::has_any_role, GrantsMiddleware, PermissionGuard};

mod role;

#[get("/macro_secured")]
// `proc-macro` way require specify your type. It can be an import or a full path.
#[has_any_role("Admin", "role::Role::Manager", type = "Role")]
// For the `ADMIN` or `MANAGER` - endpoint will give the HTTP status 200, otherwise - 403
async fn macro_secured() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[get("/manual")]
// An example of programmable protection with custom type
async fn manual_secure(details: AuthDetails<Role>) -> HttpResponse {
    if details.has_role(&Role::Admin) {
        return HttpResponse::Ok().body("Hello Admin!");
    }
    HttpResponse::Ok().body("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = GrantsMiddleware::with_extractor(extract);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .service(macro_secured)
            .service(manual_secure)
            // An example of `Guard` protection with custom Enum
            .service(
                web::resource("/guard_admin")
                    .to(|| async { HttpResponse::Ok().finish() })
                    .guard(PermissionGuard::new(Admin)),
            )
    })
    .bind("localhost:8082")?
    .workers(1)
    .run()
    .await
}

// You can specify any of your own type (`PartialEq` + `Clone`) for the return type wrapped in a vector: Result<Vec<YOUR_TYPE_HERE>, Error>
async fn extract(_req: &mut ServiceRequest) -> Result<Vec<Role>, Error> {
    // Here is a place for your code to get user permissions/grants/permissions from a request
    // For example from a token or database

    // Stub example
    Ok(vec![Role::Admin])
}
