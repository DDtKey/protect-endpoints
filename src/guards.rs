use crate::authorities::{AuthDetails, AuthoritiesCheck};
use actix_web::dev::RequestHead;
use actix_web::guard::Guard;

/// Implementation of Guard trait for validate authoritites
/// ```
/// use actix_web::dev::ServiceRequest;
/// use actix_web::{web, App, Error, HttpResponse, HttpServer};
///
/// use actix_web_grants::{GrantsMiddleware, AuthorityGuard};
/// use std::sync::Arc;
///
/// fn main() {
///     HttpServer::new(|| {
///         App::new()
///             .wrap(GrantsMiddleware::fn_extractor(extract))
///             .service(web::resource("/admin")
///                     .to(|| async { HttpResponse::Ok().finish() })
///                     .guard(AuthorityGuard::new("ROLE_ADMIN".to_string())))
///     });
/// }
///
/// async fn extract(_req: Arc<ServiceRequest>) -> Result<Vec<String>, Error> {
///    // Here is a place for your code to get user authoritites/grants/permissions from a request
///    // For example from a token or database
///
///    // Stub example
///    Ok(vec!["ROLE_ADMIN".to_string()])
/// }
/// ```
pub struct AuthorityGuard {
    allow_authority: String,
}

impl AuthorityGuard {
    pub fn new(allow_authoritites: String) -> AuthorityGuard {
        AuthorityGuard {
            allow_authority: allow_authoritites,
        }
    }
}

impl Guard for AuthorityGuard {
    fn check(&self, request: &RequestHead) -> bool {
        request
            .extensions()
            .get::<AuthDetails>()
            .filter(|details| details.has_authority(self.allow_authority.as_str()))
            .is_some()
    }
}
