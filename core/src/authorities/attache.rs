/// Allows you to transfer authorities to a web framework.
///
/// # Example
///
/// ```rust,ignore
/// use std::collections::HashSet;
///
/// // You can use you own type/enum instead of `String`
/// fn handle(req: &Request) {
///     let authorities: HashSet<String> = ...;
///     req.attach(authorities);
/// }
///
/// ```
///
pub trait AttachAuthorities<Type> {
    fn attach(&mut self, authorities: impl IntoIterator<Item = Type>);
}

#[cfg(feature = "http")]
impl<Type, Body> AttachAuthorities<Type> for http::Request<Body>
where
    Type: Eq + std::hash::Hash + Send + Sync + 'static,
{
    fn attach(&mut self, authorities: impl IntoIterator<Item = Type>) {
        self.extensions_mut()
            .insert(super::AuthDetails::new(authorities));
    }
}
