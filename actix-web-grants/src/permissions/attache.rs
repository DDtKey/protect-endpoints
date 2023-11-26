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
/// // You can use you own type/enum instead of `String`
/// fn attach(req: &ServiceRequest, permissions: Vec<String>) {
///     req.attach(permissions);
/// }
///
/// ```
///
/// [`actix-web-grants`]: crate
/// [`ServiceRequest`]: actix_web::dev::ServiceRequest
pub trait AttachPermissions<Type> {
    fn attach(&self, permissions: Vec<Type>);
}

impl<Type: PartialEq + Clone + 'static> AttachPermissions<Type> for ServiceRequest {
    fn attach(&self, permissions: Vec<Type>) {
        self.extensions_mut().insert(AuthDetails::new(permissions));
    }
}
