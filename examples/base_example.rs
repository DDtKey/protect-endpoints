use actix_web::dev::ServiceRequest;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};

use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};
use actix_web_grants::{proc_macro::has_permissions, GrantsMiddleware, PermissionGuard};

const ROLE_ADMIN: &str = "ROLE_ADMIN";
const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[get("/admin")]
#[has_permissions("OP_READ_ADMIN_INFO")]
// An example of protection via `proc-macro`
async fn macro_secured() -> HttpResponse {
    HttpResponse::Ok().body(ADMIN_RESPONSE)
}

#[get("/")]
// An example of programmable protection
async fn manual_secure(details: AuthDetails) -> HttpResponse {
    if details.has_permission(ROLE_ADMIN) {
        return HttpResponse::Ok().body(ADMIN_RESPONSE);
    }
    HttpResponse::Ok().body(OTHER_RESPONSE)
}

struct User {
    id: i32,
}

#[get("/resource/{user_id}")]
#[has_permissions("ROLE_ADMIN", secure = "user_id.into_inner() == user.id")]
// An example of protection via `proc-macro` with secure attribute
async fn secure_with_params(user_id: web::Path<i32>, user: web::Data<User>) -> HttpResponse {
    HttpResponse::Ok().body(ADMIN_RESPONSE)
}

#[actix_web::main]
// Sample application with grant protection based on extracting by your custom function
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let auth = GrantsMiddleware::with_extractor(extract);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .service(manual_secure)
            .service(macro_secured)
            // An example of `Guard` protection
            .service(
                web::resource("/guard_admin")
                    .to(|| async { HttpResponse::Ok().finish() })
                    .guard(PermissionGuard::new(ROLE_ADMIN.to_string())),
            )
            // An example with the secure attribute of macro
            .service(secure_with_params)
    })
    .bind("localhost:8081")?
    .workers(1)
    .run()
    .await
}

async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    // Here is a place for your code to get user permissions/grants/permissions from a request
    // For example from a token or database

    // Stub example
    Ok(vec![ROLE_ADMIN.to_string()])
}
