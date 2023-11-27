use crate::authorities::AttachAuthorities;
use crate::authorities::AuthoritiesExtractor;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use std::future::{self, Future, Ready};
use std::marker::PhantomData;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

/// Built-in middleware for extracting user authorities.
///
///
/// # Examples
/// ```
/// use actix_web::dev::ServiceRequest;
/// use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};
///
/// use actix_web_grants::authorities::{AuthDetails, AuthoritiesCheck};
/// use actix_web_grants::{protect, GrantsMiddleware};
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
/// // Furthermore, you can use you own type instead of `String` (e.g. Enum).
/// async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
///    // Here is a place for your code to get user permissions/roles/authorities from a request
///    // For example from a token or database
///
///    // Stub example
///    Ok(vec!["ROLE_ADMIN".to_string()])
/// }
///
/// // `proc-macro` crate has additional features, like ABAC security and custom types. See examples and `proc-macro` crate docs.
/// #[get("/admin")]
/// #[protect("ROLE_ADMIN")]
/// async fn you_service() -> impl Responder {
///     HttpResponse::Ok().finish()
/// }
/// ```
pub struct GrantsMiddleware<E, Req, Type>
where
    for<'a> E: AuthoritiesExtractor<'a, Req, Type>,
    Type: PartialEq + Clone + 'static,
{
    extractor: Rc<E>,
    phantom_req: PhantomData<Req>,
    phantom_type: PhantomData<Type>,
}

impl<E, Req, Type> GrantsMiddleware<E, Req, Type>
where
    for<'a> E: AuthoritiesExtractor<'a, Req, Type>,
    Type: PartialEq + Clone + 'static,
{
    /// Create middleware by [`AuthoritiesExtractor`].
    ///
    /// You can use a built-in implementation for `async fn` with a suitable signature (see example below).
    /// Or you can define your own implementation of trait.
    ///
    /// # Example of function with implementation of [`AuthoritiesExtractor`]
    /// ```
    /// use actix_web::dev::ServiceRequest;
    /// use actix_web::Error;
    ///
    /// async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
    ///     // Here is a place for your code to get user permissions/roles/authorities from a request
    ///      // For example from a token or database
    ///     Ok(vec!["WRITE_ACCESS".to_string()])
    /// }
    ///
    /// // Or with you own type:
    /// #[derive(PartialEq, Clone)] // required bounds
    /// enum Permission { WRITE, READ }
    /// async fn extract_enum(_req: &ServiceRequest) -> Result<Vec<Permission>, Error> {
    ///     // Here is a place for your code to get user permissions/roles/authorities from a request
    ///      // For example from a token, database or external service
    ///     Ok(vec![Permission::WRITE])
    /// }
    /// ```
    ///
    ///[`AuthoritiesExtractor`]: crate::authorities::AuthoritiesExtractor
    pub fn with_extractor(extractor: E) -> GrantsMiddleware<E, Req, Type> {
        GrantsMiddleware {
            extractor: Rc::new(extractor),
            phantom_req: PhantomData,
            phantom_type: PhantomData,
        }
    }
}

impl<S, B, E, Req, Type> Transform<S, ServiceRequest> for GrantsMiddleware<E, Req, Type>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    for<'a> E: AuthoritiesExtractor<'a, Req, Type> + 'static,
    Type: PartialEq + Clone + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = GrantsService<S, E, Req, Type>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ready(Ok(GrantsService {
            service: Rc::new(service),
            extractor: self.extractor.clone(),
            phantom_req: PhantomData,
            phantom_type: PhantomData,
        }))
    }
}

pub struct GrantsService<S, E, Req, Type>
where
    for<'a> E: AuthoritiesExtractor<'a, Req, Type> + 'static,
{
    service: Rc<S>,
    extractor: Rc<E>,
    phantom_req: PhantomData<Req>,
    phantom_type: PhantomData<Type>,
}

impl<S, B, E, Req, Type> Service<ServiceRequest> for GrantsService<S, E, Req, Type>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    for<'a> E: AuthoritiesExtractor<'a, Req, Type>,
    Type: PartialEq + Clone + 'static,
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
            let authorities: Vec<Type> = extractor.extract(&mut req).await?;
            req.attach(authorities);

            service.call(req).await
        })
    }
}
