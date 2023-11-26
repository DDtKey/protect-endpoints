use crate::authorities::{AuthDetails, AuthDetailsWrapper};
use rocket::Request;
use std::hash::Hash;

/// Allows you to transfer permissions to [`rocket-grants`] from your custom fairing.
///
/// The default implementation is provided for the &mut [`&mut Request`]
///
/// # Example
///
/// ```
/// use rocket_grants::authorities::AttachAuthorities;
/// use std::collections::HashSet;
///
/// // You can use you own type/enum instead of `String`
/// fn attach(mut req: &mut rocket::Request, authorities: Option<HashSet<String>>) {
///     req.attach(authorities);
/// }
///
/// ```
///
/// [`rocket-grants`]: crate
/// [`&mut Request`]: rocket::Request
pub trait AttachAuthorities<Type> {
    fn attach(&mut self, authorities: Option<impl IntoIterator<Item = Type>>);
}

impl<'r, Type: Eq + Hash + Send + Sync + 'static> AttachAuthorities<Type> for &mut Request<'r> {
    fn attach(&mut self, authorities: Option<impl IntoIterator<Item = Type>>) {
        let auth_details = authorities
            .map(AuthDetails::new)
            .map(|details| AuthDetailsWrapper(Some(details)))
            .unwrap_or(AuthDetailsWrapper(None));
        self.local_cache(|| auth_details);
    }
}
