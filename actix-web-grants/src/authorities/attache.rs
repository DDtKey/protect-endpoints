use crate::authorities::AuthDetails;
use actix_web::dev::ServiceRequest;
use actix_web::HttpMessage;
use std::hash::Hash;

/// Allows you to transfer authorities to [`actix-web-grants`] from your custom middleware.
///
/// The default implementation is provided for the [`ServiceRequest`]
///
/// # Example
///
/// ```
/// use std::collections::HashSet;
/// use actix_web_grants::authorities::AttachAuthorities;
/// use actix_web::dev::ServiceRequest;
///
/// // You can use you own type/enum instead of `String`
/// fn attach(req: &ServiceRequest, authorities: HashSet<String>) {
///     req.attach(authorities);
/// }
///
/// ```
///
/// [`actix-web-grants`]: crate
/// [`ServiceRequest`]: actix_web::dev::ServiceRequest
pub trait AttachAuthorities<Type> {
    fn attach(&self, authorities: impl IntoIterator<Item = Type>);
}

impl<Type: Eq + Hash + 'static> AttachAuthorities<Type> for ServiceRequest {
    fn attach(&self, authorities: impl IntoIterator<Item = Type>) {
        self.extensions_mut().insert(AuthDetails::new(authorities));
    }
}
