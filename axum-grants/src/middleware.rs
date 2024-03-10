use axum::extract::Request;
use axum::response::IntoResponse;
use protect_endpoints_core::authorities::extractor::AuthoritiesExtractor;
use protect_endpoints_core::tower::middleware::TowerGrantsMiddleware;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use tower::Layer;

pub struct GrantsLayer<Extractor, Type, Err> {
    extractor: Arc<Extractor>,
    phantom_ty: PhantomData<Type>,
    phantom_err: PhantomData<Err>,
}

impl<S, Extractor, Type, Err> Layer<S> for GrantsLayer<Extractor, Type, Err>
where
    for<'a> Extractor: AuthoritiesExtractor<'a, Request, Type, Err>,
    Type: Eq + Hash + 'static,
    Err: IntoResponse,
{
    type Service = TowerGrantsMiddleware<S, Request, Extractor, Type, Err>;

    fn layer(&self, inner: S) -> Self::Service {
        TowerGrantsMiddleware::new(inner, self.extractor.clone())
    }
}

impl<Extractor, Type, Err> GrantsLayer<Extractor, Type, Err>
where
    for<'a> Extractor: AuthoritiesExtractor<'a, Request, Type, Err>,
    Type: Eq + Hash + 'static,
    Err: IntoResponse,
{
    /// A [`tower::Layer`] with [`AuthoritiesExtractor`] .
    ///
    /// You can use a built-in implementation for `async fn` with a suitable signature (see example below).
    /// Or you can define your own implementation of trait.
    ///
    /// # Example of function with implementation of [`AuthoritiesExtractor`]
    /// ```
    /// use axum::extract::Request;
    /// use axum::response::Response;
    /// use std::collections::HashSet;
    ///
    /// async fn extract(_req: &mut Request) -> Result<HashSet<String>, Response> {
    ///     // Here is a place for your code to get user permissions/roles/authorities from a request
    ///      // For example from a token or database
    ///     Ok(HashSet::from(["WRITE_ACCESS".to_string()]))
    /// }
    ///
    /// // Or with you own type:
    /// #[derive(Eq, PartialEq, Hash)] // required bounds
    /// enum Permission { WRITE, READ }
    ///
    /// async fn extract_enum(_req: &mut Request) -> Result<HashSet<Permission>, Response> {
    ///     // Here is a place for your code to get user permissions/roles/authorities from a request
    ///      // For example from a token, database or external service
    ///     Ok(HashSet::from([Permission::WRITE]))
    /// }
    /// ```
    ///
    ///[`AuthoritiesExtractor`]: protect_endpoints_core::authorities::extractor::AuthoritiesExtractor
    pub fn with_extractor(extractor: Extractor) -> GrantsLayer<Extractor, Type, Err> {
        GrantsLayer {
            extractor: Arc::new(extractor),
            phantom_ty: PhantomData,
            phantom_err: PhantomData,
        }
    }
}

impl<Extractor, Type, Err> Clone for GrantsLayer<Extractor, Type, Err> {
    fn clone(&self) -> Self {
        GrantsLayer {
            extractor: self.extractor.clone(),
            phantom_ty: PhantomData,
            phantom_err: PhantomData,
        }
    }
}
