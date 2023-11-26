use crate::permissions::AuthDetails;
use poem::Request;

/// Allows you to transfer permissions to [`poem-grants`] from your custom middleware.
///
/// The default implementation is provided for the [`Request`]
///
/// # Example
///
/// ```
/// use poem_grants::permissions::AttachPermissions;
///
/// // You can use you own type/enum instead of `String`
/// fn attach(req: &mut poem::Request, permissions: Vec<String>) {
///     req.attach(permissions);
/// }
///
/// ```
///
/// [`poem-grants`]: crate
/// [`Request`]: poem::Request
pub trait AttachPermissions<Type> {
    fn attach(&mut self, permissions: Vec<Type>);
}

impl<Type: PartialEq + Clone + Send + Sync + 'static> AttachPermissions<Type> for Request {
    fn attach(&mut self, permissions: Vec<Type>) {
        self.extensions_mut().insert(AuthDetails::new(permissions));
    }
}
