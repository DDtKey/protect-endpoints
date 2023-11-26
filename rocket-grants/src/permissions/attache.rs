use crate::permissions::{AuthDetails, AuthDetailsWrapper};
use rocket::Request;
use std::hash::Hash;

/// Allows you to transfer permissions to [`rocket-grants`] from your custom fairing.
///
/// The default implementation is provided for the &mut [`&mut Request`]
///
/// # Example
///
/// ```
/// use rocket_grants::permissions::AttachPermissions;
/// use std::collections::HashSet;
///
/// // You can use you own type/enum instead of `String`
/// fn attach(mut req: &mut rocket::Request, permissions: Option<HashSet<String>>) {
///     req.attach(permissions);
/// }
///
/// ```
///
/// [`rocket-grants`]: crate
/// [`&mut Request`]: rocket::Request
pub trait AttachPermissions<Type> {
    fn attach(&mut self, permissions: Option<impl IntoIterator<Item = Type>>);
}

impl<'r, Type: Eq + Hash + Send + Sync + 'static> AttachPermissions<Type> for &mut Request<'r> {
    fn attach(&mut self, permissions: Option<impl IntoIterator<Item = Type>>) {
        let auth_details = permissions
            .map(AuthDetails::new)
            .map(|details| AuthDetailsWrapper(Some(details)))
            .unwrap_or(AuthDetailsWrapper(None));
        self.local_cache(|| auth_details);
    }
}
