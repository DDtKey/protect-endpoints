use std::collections::HashSet;
use std::future::Future;
use std::hash::Hash;

pub trait AuthoritiesExtractor<'a, Request, Type, Error> {
    type Future: Future<Output = Result<HashSet<Type>, Error>> + Send + Sync;

    fn extract(&self, request: &'a mut Request) -> Self::Future;
}

impl<'a, F, O, Request, Type, Error> AuthoritiesExtractor<'a, Request, Type, Error> for F
where
    for<'b> F: Fn(&'b mut Request) -> O,
    O: Future<Output = Result<HashSet<Type>, Error>> + Send + Sync,
    Type: Eq + Hash + 'static,
{
    type Future = O;

    fn extract(&self, req: &'a mut Request) -> Self::Future {
        (self)(req)
    }
}
