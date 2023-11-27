use crate::authorities::{AttachAuthorities, AuthoritiesExtractor};
use poem::{Endpoint, Middleware, Request};
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;

/// Built-in middleware for extracting user authorities.
///
///
/// # Examples
/// ```
/// use std::collections::HashSet;
/// use poem::{get, handler, listener::TcpListener, web::Path, Route, Server, http::StatusCode, Response};
///
/// use poem_grants::authorities::{AuthDetails, AuthoritiesCheck};
/// use poem_grants::GrantsMiddleware;
///
/// #[tokio::main]
/// async fn main() {
///     let app = Route::new().at("/your_service", get(you_service));
///     Server::new(TcpListener::bind("127.0.0.1:8081"))
///         .run(app);
/// }
///
/// // You can use both `&Request` and `&mut Request`
/// // Futhermore, you can use you own type instead of `String` (e.g. Enum).
/// async fn extract(_req: &poem::Request) -> poem::Result<HashSet<String>> {
///    // Here is a place for your code to get user permissions/roles/authorities from a request
///    // For example from a token or database
///
///    // Stub example
///    Ok(HashSet::from(["ROLE_ADMIN".to_string()]))
/// }
///
/// // `proc-macro` crate has additional features, like ABAC security and custom types. See examples and `proc-macro` crate docs.
/// #[poem_grants::protect("ROLE_ADMIN")]
/// #[poem::handler]
/// async fn you_service() -> poem::Response {
///     Response::builder().status(StatusCode::OK).finish()
/// }
/// ```
pub struct GrantsMiddleware<Extractor, Req, Type>
where
    for<'a> Extractor: AuthoritiesExtractor<'a, Req, Type> + Send + Sync,
    Type: Eq + Hash + Send + Sync + 'static,
    Req: Send + Sync,
{
    extractor: Arc<Extractor>,
    phantom_req: PhantomData<Req>,
    phantom_type: PhantomData<Type>,
}

impl<E, Req, Type> GrantsMiddleware<E, Req, Type>
where
    for<'a> E: AuthoritiesExtractor<'a, Req, Type> + Send + Sync,
    Type: Eq + Hash + Send + Sync + 'static,
    Req: Send + Sync,
{
    /// Create middleware by [`AuthoritiesExtractor`].
    ///
    /// You can use a built-in implementation for `async fn` with a suitable signature (see example below).
    /// Or you can define your own implementation of trait.
    ///
    /// # Example of function with implementation of [`AuthoritiesExtractor`]
    /// ```
    /// use std::collections::HashSet;
    ///
    /// async fn extract(_req: &poem::Request) -> poem::Result<HashSet<String>> {
    ///     // Here is a place for your code to get user permissions/roles/authorities from a request
    ///      // For example from a token or database
    ///     Ok(HashSet::from(["WRITE_ACCESS".to_string()]))
    /// }
    ///
    /// // Or with you own type:
    /// #[derive(Eq, PartialEq, Hash)] // required bounds
    /// enum Permission { WRITE, READ }
    /// async fn extract_enum(_req: &poem::Request) -> poem::Result<HashSet<Permission>> {
    ///     // Here is a place for your code to get user permissions/roles/authorities from a request
    ///      // For example from a token, database or external service
    ///     Ok(HashSet::from([Permission::WRITE]))
    /// }
    /// ```
    ///
    ///[`AuthoritiesExtractor`]: crate::authorities::AuthoritiesExtractor
    pub fn with_extractor(extractor: E) -> GrantsMiddleware<E, Req, Type> {
        GrantsMiddleware {
            extractor: Arc::new(extractor),
            phantom_req: PhantomData,
            phantom_type: PhantomData,
        }
    }
}

/// Endpoint for GrantsMiddleware.
pub struct GrantsEndpoint<End, Extractor, Req, Type>
where
    End: Endpoint,
    for<'a> Extractor: AuthoritiesExtractor<'a, Req, Type> + Send + Sync,
    Type: Eq + Hash + Send + Sync + 'static,
    Req: Send + Sync,
{
    inner: End,
    extractor: Arc<Extractor>,
    phantom_req: PhantomData<Req>,
    phantom_type: PhantomData<Type>,
}

impl<End, Extractor, Req, Type> Middleware<End> for GrantsMiddleware<Extractor, Req, Type>
where
    End: Endpoint,
    for<'a> Extractor: AuthoritiesExtractor<'a, Req, Type> + Send + Sync,
    Type: Eq + Hash + Send + Sync + 'static,
    Req: Send + Sync,
{
    type Output = GrantsEndpoint<End, Extractor, Req, Type>;

    fn transform(&self, ep: End) -> Self::Output {
        GrantsEndpoint {
            inner: ep,
            extractor: self.extractor.clone(),
            phantom_req: PhantomData,
            phantom_type: PhantomData,
        }
    }
}

#[poem::async_trait]
impl<End, Extractor, Req, Type> Endpoint for GrantsEndpoint<End, Extractor, Req, Type>
where
    End: Endpoint,
    for<'a> Extractor: AuthoritiesExtractor<'a, Req, Type> + Send + Sync,
    Type: Eq + Hash + Send + Sync + 'static,
    Req: Send + Sync,
{
    type Output = End::Output;

    async fn call(&self, mut req: Request) -> poem::Result<Self::Output> {
        let authorities: HashSet<Type> = self.extractor.extract(&mut req).await?;
        req.attach(authorities);

        self.inner.call(req).await
    }
}
