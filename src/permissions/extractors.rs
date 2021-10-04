use actix_web::dev::ServiceRequest;
use actix_web::Error;
use std::future::Future;

pub trait PermissionsExtractor<'a, Req> {
    type Future: Future<Output = Result<Vec<String>, Error>>;

    fn extract(&self, request: &'a mut ServiceRequest) -> Self::Future;
}

impl<'a, F, O> PermissionsExtractor<'a, &ServiceRequest> for F
where
    F: Fn(&'a ServiceRequest) -> O,
    O: Future<Output = Result<Vec<String>, Error>>,
{
    type Future = O;

    fn extract(&self, req: &'a mut ServiceRequest) -> Self::Future {
        (self)(req)
    }
}

impl<'a, F, O> PermissionsExtractor<'a, &mut ServiceRequest> for F
where
    F: Fn(&'a mut ServiceRequest) -> O,
    O: Future<Output = Result<Vec<String>, Error>>,
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

    async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
        Ok(vec!["TEST_PERMISSION".to_string()])
    }

    #[actix_rt::test]
    async fn test_fn_extractor_impl() {
        let mut req = test::TestRequest::get().to_srv_request();
        let permissions = extract.extract(&mut req).await;

        permissions
            .unwrap()
            .iter()
            .for_each(|perm| assert_eq!("TEST_PERMISSION", perm.as_str()));
    }

    async fn mut_extract(_req: &mut ServiceRequest) -> Result<Vec<String>, Error> {
        Ok(vec!["TEST_PERMISSION".to_string()])
    }

    #[actix_rt::test]
    async fn test_fn_mut_extractor_impl() {
        let mut req = test::TestRequest::get().to_srv_request();
        let permissions = mut_extract.extract(&mut req).await;

        permissions
            .unwrap()
            .iter()
            .for_each(|perm| assert_eq!("TEST_PERMISSION", perm.as_str()));
    }
}
