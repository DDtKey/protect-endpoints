use actix_web::dev::ServiceRequest;
use actix_web::Error;
use std::collections::HashSet;
use std::future::Future;
use std::hash::Hash;

pub trait AuthoritiesExtractor<'a, Req, Type> {
    type Future: Future<Output = Result<HashSet<Type>, Error>>;

    fn extract(&self, request: &'a mut ServiceRequest) -> Self::Future;
}

impl<'a, F, O, Type> AuthoritiesExtractor<'a, &ServiceRequest, Type> for F
where
    F: Fn(&'a ServiceRequest) -> O,
    O: Future<Output = Result<HashSet<Type>, Error>>,
    Type: Eq + Hash + 'static,
{
    type Future = O;

    fn extract(&self, req: &'a mut ServiceRequest) -> Self::Future {
        (self)(req)
    }
}

impl<'a, F, O, Type> AuthoritiesExtractor<'a, &mut ServiceRequest, Type> for F
where
    F: Fn(&'a mut ServiceRequest) -> O,
    O: Future<Output = Result<HashSet<Type>, Error>>,
    Type: Eq + Hash + 'static,
{
    type Future = O;

    fn extract(&self, req: &'a mut ServiceRequest) -> Self::Future {
        (self)(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::dev::ServiceRequest;
    use actix_web::test;

    async fn extract(_req: &ServiceRequest) -> Result<HashSet<String>, Error> {
        Ok(HashSet::from(["TEST_PERMISSION".to_string()]))
    }

    #[actix_rt::test]
    async fn test_fn_extractor_impl() {
        let mut req = test::TestRequest::get().to_srv_request();
        let authorities = extract.extract(&mut req).await;

        authorities
            .unwrap()
            .iter()
            .for_each(|authority| assert_eq!("TEST_PERMISSION", authority.as_str()));
    }

    async fn mut_extract(_req: &mut ServiceRequest) -> Result<HashSet<String>, Error> {
        Ok(HashSet::from(["TEST_PERMISSION".to_string()]))
    }

    #[actix_rt::test]
    async fn test_fn_mut_extractor_impl() {
        let mut req = test::TestRequest::get().to_srv_request();
        let authorities = mut_extract.extract(&mut req).await;

        authorities
            .unwrap()
            .iter()
            .for_each(|authority| assert_eq!("TEST_PERMISSION", authority.as_str()));
    }
}
