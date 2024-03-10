use crate::authorities::{extractor::AuthoritiesExtractor, AttachAuthorities};
use futures_util::future::BoxFuture;
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::{future::Future, pin::Pin};
use tower::Service;

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

impl<S: Clone, Request, Extractor, Type, Error> Clone
    for TowerGrantsMiddleware<S, Request, Extractor, Type, Error>
{
    fn clone(&self) -> Self {
        Self::new(self.inner.clone(), self.extractor.clone())
    }
}
