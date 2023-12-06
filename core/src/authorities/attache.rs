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
    fn attach(&self, authorities: impl IntoIterator<Item = Type>);
}
