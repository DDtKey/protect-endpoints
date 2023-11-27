use crate::authorities::{AuthDetails, AuthoritiesCheck};
use actix_web::guard::{Guard, GuardContext};
use std::hash::Hash;

/// Implementation of Guard trait for validate authorities
/// ```
/// use actix_web::dev::ServiceRequest;
/// use actix_web::{web, App, Error, HttpResponse, HttpServer};
///
/// use actix_web_grants::{GrantsMiddleware, AuthorityGuard};
/// use std::sync::Arc;
/// use std::collections::HashSet;
///
/// fn main() {
///     HttpServer::new(|| {
///         App::new()
///             .wrap(GrantsMiddleware::with_extractor(extract))
///             .service(web::resource("/admin")
///                     .to(|| async { HttpResponse::Ok().finish() })
///                     .guard(AuthorityGuard::new("ROLE_ADMIN".to_string())))
///     });
/// }
///
/// async fn extract(_req: &ServiceRequest) -> Result<HashSet<String>, Error> {
///    // Here is a place for your code to get user permissions/roles/authorities from a request
///    // For example from a token or database
///
///    // Stub example
///    Ok(HashSet::from(["ROLE_ADMIN".to_string()]))
/// }
/// ```
pub struct AuthorityGuard<Type> {
    allow_authority: Type,
}

impl<Type: Eq + Hash + 'static> AuthorityGuard<Type> {
    pub fn new(allow_authority: Type) -> AuthorityGuard<Type> {
        AuthorityGuard { allow_authority }
    }
}

impl<Type: Eq + Hash + 'static> Guard for AuthorityGuard<Type> {
    fn check(&self, request: &GuardContext) -> bool {
        request
            .req_data()
            .get::<AuthDetails<Type>>()
            .filter(|details| details.has_authority(&self.allow_authority))
            .is_some()
    }
}
