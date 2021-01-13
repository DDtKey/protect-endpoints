use actix_web::dev::ServiceRequest;
use actix_web::Error;
use std::future::Future;
use std::sync::Arc;

pub trait AuthoritiesExtractor {
    type Future: Future<Output = Result<Vec<String>, Error>>;

    fn extract(&self, request: Arc<ServiceRequest>) -> Self::Future;
}

pub struct FnAuthoritiesExtractor<F, O>
where
    F: Fn(Arc<ServiceRequest>) -> O,
    O: Future<Output = Result<Vec<String>, Error>>,
{
    extract_fn: F,
}

impl<F, O> FnAuthoritiesExtractor<F, O>
where
    F: Fn(Arc<ServiceRequest>) -> O,
    O: Future<Output = Result<Vec<String>, Error>>,
{
    pub fn new(extract_fn: F) -> FnAuthoritiesExtractor<F, O> {
        FnAuthoritiesExtractor { extract_fn }
    }
}

impl<F, O> AuthoritiesExtractor for FnAuthoritiesExtractor<F, O>
where
    F: Fn(Arc<ServiceRequest>) -> O,
    O: Future<Output = Result<Vec<String>, Error>>,
{
    type Future = O;

    fn extract(&self, request: Arc<ServiceRequest>) -> Self::Future {
        (self.extract_fn)(request)
    }
}
