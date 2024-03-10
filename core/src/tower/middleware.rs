use crate::authorities::{extractor::AuthoritiesExtractor, AttachAuthorities};
use futures_util::future::BoxFuture;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::{future::Future, pin::Pin};
use tower::{Layer, Service};

pub struct GrantsLayer<Extractor, Request, Type, Err> {
    extractor: Arc<Extractor>,
    phantom_req: PhantomData<Request>,
    phantom_ty: PhantomData<Type>,
    phantom_err: PhantomData<Err>,
}

pub struct TowerGrantsMiddleware<S, Request, Extractor, Type, Error> {
    inner: S,
    extractor: Arc<Extractor>,
    phantom_req: PhantomData<Request>,
    phantom_type: PhantomData<Type>,
    phantom_error: PhantomData<Error>,
}

impl<S, Request, Extractor, Type, Error> TowerGrantsMiddleware<S, Request, Extractor, Type, Error> {
    pub fn new(inner: S, extractor: Arc<Extractor>) -> Self {
        Self {
            inner,
            extractor,
            phantom_req: PhantomData,
            phantom_type: PhantomData,
            phantom_error: PhantomData,
        }
    }
}

#[pin_project::pin_project]
pub struct ResponseFuture<Output> {
    #[pin]
    future: BoxFuture<'static, Output>,
}

impl<S, Request, RespBody, Extractor, Type, Error> Service<Request>
    for TowerGrantsMiddleware<S, Request, Extractor, Type, Error>
where
    S::Future: Send,
    Type: Eq + Hash + Send + Sync + 'static,
    S: Service<Request, Response = http::Response<RespBody>> + Clone + Send + Sync + 'static,
    Request: AttachAuthorities<Type> + Send + 'static,
    Error: Send + Into<http::Response<RespBody>>,
    for<'a> Extractor: AuthoritiesExtractor<'a, Request, Type, Error> + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut request: Request) -> Self::Future {
        let mut inner = self.inner.clone();
        let extractor = self.extractor.clone();
        let future = Box::pin(async move {
            let authorities = match extractor.extract(&mut request).await {
                Ok(res) => res,
                Err(err) => return Ok(err.into()),
            };
            request.attach(authorities);

            inner.call(request).await
        });

        ResponseFuture { future }
    }
}

impl<Output> Future for ResponseFuture<Output> {
    type Output = Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        this.future.poll(cx)
    }
}

impl<S, Request, Extractor, Type, Err> Layer<S> for GrantsLayer<Extractor, Request, Type, Err>
where
    for<'a> Extractor: AuthoritiesExtractor<'a, Request, Type, Err>,
    Type: Eq + Hash + 'static,
{
    type Service = TowerGrantsMiddleware<S, Request, Extractor, Type, Err>;

    fn layer(&self, inner: S) -> Self::Service {
        TowerGrantsMiddleware::new(inner, self.extractor.clone())
    }
}

impl<Extractor, Request, Type, Err> GrantsLayer<Extractor, Request, Type, Err>
where
    for<'a> Extractor: AuthoritiesExtractor<'a, Request, Type, Err>,
    Type: Eq + Hash + 'static,
{
    /// A [`tower::Layer`] with [`AuthoritiesExtractor`] .
    ///
    /// You can use a built-in implementation for `async fn` with a suitable signature (see example below).
    /// Or you can define your own implementation of trait.
    ///
    /// # Example of function with implementation of [`AuthoritiesExtractor`]
    /// ```
    /// use http::Request;
    /// use http::Response;
    /// use std::collections::HashSet;
    ///
    /// async fn extract(_req: &mut Request<String>) -> Result<HashSet<String>, Response<String>> {
    ///     // Here is a place for your code to get user permissions/roles/authorities from a request
    ///      // For example from a token or database
    ///     Ok(HashSet::from(["WRITE_ACCESS".to_string()]))
    /// }
    ///
    /// // Or with you own type:
    /// #[derive(Eq, PartialEq, Hash)] // required bounds
    /// enum Permission { WRITE, READ }
    ///
    /// async fn extract_enum(_req: &mut Request<String>) -> Result<HashSet<Permission>, Response<String>> {
    ///     // Here is a place for your code to get user permissions/roles/authorities from a request
    ///      // For example from a token, database or external service
    ///     Ok(HashSet::from([Permission::WRITE]))
    /// }
    /// ```
    ///
    ///[`AuthoritiesExtractor`]: crate::authorities::extractor::AuthoritiesExtractor
    pub fn with_extractor(extractor: Extractor) -> GrantsLayer<Extractor, Request, Type, Err> {
        GrantsLayer {
            extractor: Arc::new(extractor),
            phantom_req: PhantomData,
            phantom_ty: PhantomData,
            phantom_err: PhantomData,
        }
    }
}

impl<Extractor, Request, Type, Err> Clone for GrantsLayer<Extractor, Request, Type, Err> {
    fn clone(&self) -> Self {
        GrantsLayer {
            extractor: self.extractor.clone(),
            phantom_req: PhantomData,
            phantom_ty: PhantomData,
            phantom_err: PhantomData,
        }
    }
}

impl<S: Clone, Request, Extractor, Type, Error> Clone
    for TowerGrantsMiddleware<S, Request, Extractor, Type, Error>
{
    fn clone(&self) -> Self {
        Self::new(self.inner.clone(), self.extractor.clone())
    }
}
