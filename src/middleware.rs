use crate::permissions::AttachPermissions;
use crate::permissions::PermissionsExtractor;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use std::future::{self, Future, Ready};
use std::marker::PhantomData;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

/// Built-in middleware for extracting user permission.
///
///
/// # Examples
/// ```
/// use actix_web::dev::ServiceRequest;
/// use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};
///
/// use actix_web_grants::permissions::{AuthDetails, PermissionsCheck};
/// use actix_web_grants::{proc_macro::has_permissions, GrantsMiddleware};
///
/// fn main() {
///     HttpServer::new(|| {
///         let auth = GrantsMiddleware::with_extractor(extract);
///         App::new()
///             .wrap(auth)
///             .service(you_service)
///     });
/// }
///
/// // You can use both &ServiceRequest and &mut ServiceRequest
/// async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
///    // Here is a place for your code to get user permissions/grants/permissions from a request
///    // For example from a token or database
///
///    // Stub example
///    Ok(vec!["ROLE_ADMIN".to_string()])
/// }
///
/// // `has_permissions` is one of options to validate permissions.
/// #[get("/admin")]
/// #[has_permissions("ROLE_ADMIN")]
/// async fn you_service() -> impl Responder {
///     HttpResponse::Ok().finish()
/// }
/// ```
pub struct GrantsMiddleware<T, Req>
where
    for<'a> T: PermissionsExtractor<'a, Req>,
{
    extractor: Rc<T>,
    phantom_req: PhantomData<Req>,
}

impl<T, Req> GrantsMiddleware<T, Req>
where
    for<'a> T: PermissionsExtractor<'a, Req>,
{
    /// Create middleware by [`PermissionsExtractor`].
    ///
    /// You can use a built-in implementation for `async fn` with a suitable signature (see example below).
    /// Or you can define your own implementation of trait.
    ///
    /// # Example of function with implementation of [`PermissionsExtractor`]
    /// ```
    /// use actix_web::dev::ServiceRequest;
    /// use actix_web::Error;
    ///
    /// async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    ///     // Here is a place for your code to get user permissions/grants/permissions from a request
    ///      // For example from a token or database
    ///     Ok(vec!["WRITE_ACCESS".to_string()])
    /// }
    /// ```
    ///
    ///[`PermissionsExtractor`]: crate::permissions::PermissionsExtractor
    pub fn with_extractor(extractor: T) -> GrantsMiddleware<T, Req> {
        GrantsMiddleware {
            extractor: Rc::new(extractor),
            phantom_req: PhantomData,
        }
    }
}

impl<S, B, T, Req> Transform<S, ServiceRequest> for GrantsMiddleware<T, Req>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    for<'a> T: PermissionsExtractor<'a, Req> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = GrantsService<S, T, Req>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(GrantsService {
            service: Rc::new(service),
            extractor: self.extractor.clone(),
            phantom_req: PhantomData,
        }))
    }
}

pub struct GrantsService<S, T, Req>
where
    for<'a> T: PermissionsExtractor<'a, Req> + 'static,
{
    service: Rc<S>,
    extractor: Rc<T>,
    phantom_req: PhantomData<Req>,
}

impl<S, B, T, Req> Service<ServiceRequest> for GrantsService<S, T, Req>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    for<'a> T: PermissionsExtractor<'a, Req>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Error>>>>;

    fn poll_ready(&self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let extractor = Rc::clone(&self.extractor);

        Box::pin(async move {
            let permissions: Vec<String> = extractor.extract(&mut req).await?;
            req.attach(permissions);

            service.call(req).await
        })
    }
}
