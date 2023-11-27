use poem::Request;
use std::future::Future;

pub trait AuthoritiesExtractor<'a, Req, Type> {
    type Future: Future<Output = poem::Result<Vec<Type>>> + Send + Sync;

    fn extract(&self, request: &'a mut Request) -> Self::Future;
}

impl<'a, F, O, Type> AuthoritiesExtractor<'a, &Request, Type> for F
where
    F: Fn(&'a Request) -> O,
    O: Future<Output = poem::Result<Vec<Type>>> + Send + Sync,
    Type: PartialEq + Clone + 'static,
{
    type Future = O;

    fn extract(&self, req: &'a mut Request) -> Self::Future {
        (self)(req)
    }
}

impl<'a, F, O, Type> AuthoritiesExtractor<'a, &mut Request, Type> for F
where
    F: Fn(&'a mut Request) -> O,
    O: Future<Output = poem::Result<Vec<Type>>> + Send + Sync,
    Type: PartialEq + Clone + 'static,
{
    type Future = O;

    fn extract(&self, req: &'a mut Request) -> Self::Future {
        (self)(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn extract(_req: &Request) -> poem::Result<Vec<String>> {
        Ok(vec!["TEST_PERMISSION".to_string()])
    }

    #[tokio::test]
    async fn test_fn_extractor_impl() {
        let req = Request::default();
        let authorities = extract(&req).await;

        authorities
            .unwrap()
            .iter()
            .for_each(|perm| assert_eq!("TEST_PERMISSION", perm.as_str()));
    }

    async fn mut_extract(_req: &mut Request) -> poem::Result<Vec<String>> {
        Ok(vec!["TEST_PERMISSION".to_string()])
    }

    #[tokio::test]
    async fn test_fn_mut_extractor_impl() {
        let mut req = Request::default();
        let authorities = mut_extract(&mut req).await;

        authorities
            .unwrap()
            .iter()
            .for_each(|perm| assert_eq!("TEST_PERMISSION", perm.as_str()));
    }
}
