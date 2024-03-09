use std::collections::HashSet;
use std::future::Future;
use std::hash::Hash;

pub trait AuthoritiesExtractor<'a, Request, Type, Error> {
    type Future: Future<Output = Result<HashSet<Type>, Error>> + Send + Sync;

    fn extract(&self, request: &'a mut Request) -> Self::Future;
}

impl<'a, F, O, Request, Type, Error> AuthoritiesExtractor<'a, Request, Type, Error> for F
where
    F: Fn(&'a mut Request) -> O,
    Request: 'a,
    O: Future<Output = Result<HashSet<Type>, Error>> + Send + Sync,
    Type: Eq + Hash + 'static,
{
    type Future = O;

    fn extract(&self, req: &'a mut Request) -> Self::Future {
        (self)(req)
    }
}

#[cfg(test)]
mod tests {
    use super::AuthoritiesExtractor;
    use std::collections::HashSet;

    struct FakeRequest;

    async fn extractor(_req: &mut FakeRequest) -> Result<HashSet<String>, ()> {
        Ok(HashSet::from(["TEST_PERMISSION".to_string()]))
    }

    #[tokio::test]
    async fn test_fn_mut_extractor_impl() {
        let authorities: Result<_, ()> = extractor.extract(&mut FakeRequest).await;

        authorities
            .unwrap()
            .iter()
            .for_each(|perm| assert_eq!("TEST_PERMISSION", perm.as_str()));
    }
}
