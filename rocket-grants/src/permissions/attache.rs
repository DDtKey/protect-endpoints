use crate::permissions::{AuthDetails, AuthDetailsWrapper};
use rocket::Request;

/// Allows you to transfer permissions to [`rocket-grants`] from your custom fairing.
///
/// The default implementation is provided for the &mut [`&mut Request`]
///
/// # Example
///
/// ```
/// use rocket_grants::permissions::AttachPermissions;
///
/// // You can use you own type/enum instead of `String`
/// fn attach(mut req: &mut rocket::Request, permissions: Option<Vec<String>>) {
///     req.attach(permissions);
/// }
///
/// ```
///
/// [`rocket-grants`]: crate
/// [`&mut Request`]: rocket::Request
pub trait AttachPermissions<Type> {
    fn attach(&mut self, permissions: Option<Vec<Type>>);
}

impl<'r, Type: PartialEq + Clone + Send + Sync + 'static> AttachPermissions<Type>
    for &mut Request<'r>
{
    fn attach(&mut self, permissions: Option<Vec<Type>>) {
        let auth_details = permissions
            .map(AuthDetails::new)
            .map(|details| AuthDetailsWrapper(Some(details)))
            .unwrap_or(AuthDetailsWrapper(None));
        self.local_cache(|| auth_details);
    }
}
