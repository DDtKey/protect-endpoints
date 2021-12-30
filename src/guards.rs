use crate::permissions::{AuthDetails, PermissionsCheck};
use actix_web::guard::{Guard, GuardContext};

/// Implementation of Guard trait for validate permissions
/// ```
/// use actix_web::dev::ServiceRequest;
/// use actix_web::{web, App, Error, HttpResponse, HttpServer};
///
/// use actix_web_grants::{GrantsMiddleware, PermissionGuard};
/// use std::sync::Arc;
///
/// fn main() {
///     HttpServer::new(|| {
///         App::new()
///             .wrap(GrantsMiddleware::with_extractor(extract))
///             .service(web::resource("/admin")
///                     .to(|| async { HttpResponse::Ok().finish() })
///                     .guard(PermissionGuard::new("ROLE_ADMIN".to_string())))
///     });
/// }
///
/// async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
///    // Here is a place for your code to get user permissions/grants/permissions from a request
///    // For example from a token or database
///
///    // Stub example
///    Ok(vec!["ROLE_ADMIN".to_string()])
/// }
/// ```
pub struct PermissionGuard {
    allow_permission: String,
}

impl PermissionGuard {
    pub fn new(allow_permission: String) -> PermissionGuard {
        PermissionGuard { allow_permission }
    }
}

impl Guard for PermissionGuard {
    fn check(&self, request: &GuardContext) -> bool {
        request
            .req_data()
            .get::<AuthDetails>()
            .filter(|details| details.has_permission(self.allow_permission.as_str()))
            .is_some()
    }
}
