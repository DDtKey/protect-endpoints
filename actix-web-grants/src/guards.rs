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
///                     .guard(AuthorityGuard::contains("ROLE_ADMIN".to_string())))
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

pub struct AuthorityGuard<T> {
    allow_authority: Type<T>,
}

pub enum Type<T> {
    Single(T),
    Any(Vec<T>),
    All(Vec<T>),
}

impl<T: Eq + Hash + 'static> AuthorityGuard<T> {
     pub fn create(allow_authority: Type<T>) -> AuthorityGuard<T> {
        AuthorityGuard {
            allow_authority: allow_authority,
        }
    }
    
    #[deprecated]
    pub fn new(allow_authority: T) -> AuthorityGuard<T> {
        Self::contains(allow_authority)
    }

    pub fn contains(allow_authority: T) -> AuthorityGuard<T> {
        Self::create(Type::Single(allow_authority))
    }

    pub fn all(allow_authority: impl IntoIterator<Item = T>) -> AuthorityGuard<T> {
        Self::create(Type::All(allow_authority.into_iter().collect()))
    }

    pub fn any(allow_authority: impl IntoIterator<Item = T>) -> AuthorityGuard<T> {
        Self::create(Type::Any(allow_authority.into_iter().collect()))
    }
}

impl<T: Eq + Hash + 'static> Guard for AuthorityGuard<T> {
    fn check(&self, request: &GuardContext) -> bool {
        let req_data = request.req_data();
        let details = req_data.get::<AuthDetails<T>>();
        match &self.allow_authority {
            Type::Single(s) => details
                .filter(|details| details.has_authority(&s))
                .is_some(),
            Type::Any(items) => details
                .filter(|details| details.has_any_authority(&items.iter().collect::<Vec<_>>()))
                .is_some(),
            Type::All(items) => details
                .filter(|details| details.has_authorities(&items.iter().collect::<Vec<_>>()))
                .is_some(),
        }
    }
}

