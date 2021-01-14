use crate::authorities::AuthDetails;
use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;

/// Allows you to transfer authorities to [`actix-web-grants`] from your custom middleware.
///
/// The default implementation is provided for the [`ServiceRequest`]
///
/// # Example
///
/// ```
/// use actix_web_grants::authorities::AttachAuthorities;
/// use actix_web::dev::ServiceRequest;
///
/// fn attach(req: ServiceRequest, authorities: Vec<String>) {
///     req.attach(authorities);
/// }
///
/// ```
///
/// [`actix-web-grants`]: crate
/// [`ServiceRequest`]: actix_web::dev::ServiceRequest
pub trait AttachAuthorities {
    fn attach(&self, authorities: Vec<String>);
}

impl AttachAuthorities for ServiceRequest {
    fn attach(&self, authorities: Vec<String>) {
        self.extensions_mut().insert(AuthDetails::new(authorities));
    }
}
