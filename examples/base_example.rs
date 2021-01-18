use actix_web::dev::ServiceRequest;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer};

use actix_web_grants::authorities::{AuthDetails, AuthoritiesCheck};
use actix_web_grants::{proc_macro::has_authorities, AuthorityGuard, GrantsMiddleware};

const ROLE_ADMIN: &str = "ROLE_ADMIN";
const ADMIN_RESPONSE: &str = "Hello Admin!";
const OTHER_RESPONSE: &str = "Hello!";

#[get("/admin")]
#[has_authorities("ROLE_ADMIN")]
/// An example of protection via `proc-macro`
async fn macro_secured() -> HttpResponse {
    HttpResponse::Ok().body(ADMIN_RESPONSE)
}

#[get("/")]
/// An example of programmable protection
async fn manual_secure(details: AuthDetails) -> HttpResponse {
    if details.has_authority(ROLE_ADMIN) {
        return HttpResponse::Ok().body(ADMIN_RESPONSE);
    }
    HttpResponse::Ok().body(OTHER_RESPONSE)
}

#[actix_web::main]
/// Sample application with grant protection based on extracting by your custom function
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
                    .guard(AuthorityGuard::new(ROLE_ADMIN.to_string())),
            )
    })
    .bind("localhost:8081")?
    .workers(1)
    .run()
    .await
}

async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    // Here is a place for your code to get user authorities/grants/permissions from a request
    // For example from a token or database

    // Stub example
    Ok(vec![ROLE_ADMIN.to_string()])
}
