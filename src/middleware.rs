use crate::authorities::AttachAuthorities;
use crate::authorities::{AuthoritiesExtractor, FnAuthoritiesExtractor};
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::error::ErrorInternalServerError;
use actix_web::Error;
use std::cell::RefCell;
use std::future::{self, Future, Ready};
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;
use std::task::{Context, Poll};

/// Built-in middleware for extracting user authority.
///
///
/// # Examples
/// ```
/// use actix_web::dev::ServiceRequest;
/// use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};
///
/// use actix_web_grants::authorities::{AuthDetails, AuthoritiesCheck};
/// use actix_web_grants::{proc_macro::has_authorities, GrantsMiddleware};
/// use std::sync::Arc;
///
/// fn main() {
///     HttpServer::new(|| {
///         let auth = GrantsMiddleware::fn_extractor(extract);
///         App::new()
///             .wrap(auth)
///             .service(you_service)
///     });
/// }
///
/// async fn extract(_req: Arc<ServiceRequest>) -> Result<Vec<String>, Error> {
///    // Here is a place for your code to get user authorities/grants/permissions from a request
///    // For example from a token or database
///
///    // Stub example
///    Ok(vec!["ROLE_ADMIN".to_string()])
/// }
///
/// // `has_authorities` is one of options to validate authorities.
/// #[get("/admin")]
/// #[has_authorities("ROLE_ADMIN")]
/// async fn you_service() -> impl Responder {
///     HttpResponse::Ok().finish()
/// }
/// ```
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
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        let extractor: Arc<T> = self.extractor.clone();
        let service = Rc::new(RefCell::new(service));
        future::ready(Ok(GrantsService { service, extractor }))
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
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        let service = Rc::clone(&self.service);
        let req = Arc::new(req);
        let authorities_fut = Arc::clone(&self.extractor).extract(req.clone());

        Box::pin(async move {
            let authorities: Vec<String> = authorities_fut.await?;
            req.attach(authorities);
            let req = Arc::try_unwrap(req)
                .map_err(|_| ErrorInternalServerError("Request processing error"))?;
            let fut = service.borrow_mut().call(req);
            fut.await
        })
    }
}
