use crate::permissions::AuthDetails;
use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;

/// Allows you to transfer permissions to [`actix-web-grants`] from your custom middleware.
///
/// The default implementation is provided for the [`ServiceRequest`]
///
/// # Example
///
/// ```
/// use actix_web_grants::permissions::AttachPermissions;
/// use actix_web::dev::ServiceRequest;
///
/// fn attach(req: ServiceRequest, permissions: Vec<String>) {
///     req.attach(permissions);
/// }
///
/// ```
///
/// [`actix-web-grants`]: crate
/// [`ServiceRequest`]: actix_web::dev::ServiceRequest
pub trait AttachPermissions {
    fn attach(&self, permissions: Vec<String>);
}

impl AttachPermissions for ServiceRequest {
    fn attach(&self, permissions: Vec<String>) {
        self.extensions_mut().insert(AuthDetails::new(permissions));
    }
}
