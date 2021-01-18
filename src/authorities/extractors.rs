use actix_web::dev::ServiceRequest;
use actix_web::Error;
use std::future::Future;

pub trait AuthoritiesExtractor<'a> {
    type Future: Future<Output = Result<Vec<String>, Error>>;

    fn extract(&self, request: &'a ServiceRequest) -> Self::Future;
}

impl<'a, F, O> AuthoritiesExtractor<'a> for F
where
    F: Fn(&'a ServiceRequest) -> O,
    O: Future<Output = Result<Vec<String>, Error>>,
{
    type Future = O;

    fn extract(&self, req: &'a ServiceRequest) -> Self::Future {
        (self)(req)
    }
}
