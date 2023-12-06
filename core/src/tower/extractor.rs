use std::collections::HashSet;
use std::future::Future;

pub trait AuthoritiesExtractor<'a, Request, Type, Error> {
    type Future: Future<Output = Result<HashSet<Type>, Error>> + Send + Sync;

    fn extract(&self, request: &'a mut Request) -> Self::Future;
}
