use crate::authorities::AttachAuthorities;
use crate::authorities::{AuthoritiesExtractor, FnAuthoritiesExtractor};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::ErrorInternalServerError;
use actix_web::Error;
use futures_util::future::{self, FutureExt, LocalBoxFuture};
use std::cell::RefCell;
use std::future::Future;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};

pub struct GrantsMiddleware<T>
where
    T: AuthoritiesExtractor,
{
    extractor: Arc<T>,
}

impl<T> GrantsMiddleware<T>
where
    T: AuthoritiesExtractor,
{
    pub fn with_extractor(extractor: T) -> GrantsMiddleware<T> {
        GrantsMiddleware {
            extractor: Arc::new(extractor),
        }
    }
}

impl<F, O> GrantsMiddleware<FnAuthoritiesExtractor<F, O>>
where
    F: Fn(Arc<ServiceRequest>) -> O,
    O: Future<Output = Result<Vec<String>, Error>>,
{
    pub fn fn_extractor(extract_fn: F) -> GrantsMiddleware<FnAuthoritiesExtractor<F, O>> {
        let extractor = FnAuthoritiesExtractor::new(extract_fn);
        Self::with_extractor(extractor)
    }
}

impl<S, B, T> Transform<S> for GrantsMiddleware<T>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    T: AuthoritiesExtractor + 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = GrantsService<S, T>;
    type InitError = ();
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let extractor: Arc<T> = self.extractor.clone();
        let service = Rc::new(RefCell::new(service));
        future::ok(GrantsService { service, extractor })
    }
}

pub struct GrantsService<S, T>
where
    T: AuthoritiesExtractor + 'static,
{
    service: Rc<RefCell<S>>,
    extractor: Arc<T>,
}

impl<S, B, T> Service for GrantsService<S, T>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    T: AuthoritiesExtractor,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Error>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let service = Rc::clone(&self.service);
        let req = Arc::new(req);
        let authorities_fut = Arc::clone(&self.extractor).extract(req.clone());

        async move {
            let authorities: Vec<String> = authorities_fut.await?;
            req.attach(authorities);
            let req = Arc::try_unwrap(req)
                .map_err(|_| ErrorInternalServerError("Request processing error"))?;
            let fut = service.borrow_mut().call(req);
            fut.await
        }
        .boxed_local()
    }
}
