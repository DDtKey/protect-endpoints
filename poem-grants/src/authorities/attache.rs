use crate::authorities::AuthDetails;
use poem::Request;
use std::hash::Hash;

/// Allows you to transfer authorities to [`poem-grants`] from your custom middleware.
///
/// The default implementation is provided for the [`Request`]
///
/// # Example
///
/// ```
/// use poem_grants::authorities::AttachAuthorities;
///
/// // You can use you own type/enum instead of `String`
/// fn attach(req: &mut poem::Request, authorities: Vec<String>) {
///     req.attach(authorities);
/// }
///
/// ```
///
/// [`poem-grants`]: crate
/// [`Request`]: poem::Request
pub trait AttachAuthorities<Type> {
    fn attach(&mut self, authorities: impl IntoIterator<Item = Type>);
}

impl<Type: Eq + Hash + Send + Sync + 'static> AttachAuthorities<Type> for Request {
    fn attach(&mut self, authorities: impl IntoIterator<Item = Type>) {
        self.extensions_mut().insert(AuthDetails::new(authorities));
    }
}
