use actix_web::dev::ServiceRequest;
use actix_web::Error;
use std::future::Future;

pub trait PermissionsExtractor<'a> {
    type Future: Future<Output = Result<Vec<String>, Error>>;

    fn extract(&self, request: &'a ServiceRequest) -> Self::Future;
}

impl<'a, F, O> PermissionsExtractor<'a> for F
where
    F: Fn(&'a ServiceRequest) -> O,
    O: Future<Output = Result<Vec<String>, Error>>,
{
    type Future = O;

    fn extract(&self, req: &'a ServiceRequest) -> Self::Future {
        (self)(req)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    async fn extract(_req: &ServiceRequest) -> Result<Vec<String>, Error> {
        Ok(vec!["TEST_PERMISSION".to_string()])
    }

    #[actix_rt::test]
    async fn test_fn_extractor_impl() {
        let req = test::TestRequest::get().to_srv_request();
        let permissions = extract.extract(&req).await;

        permissions
            .unwrap()
            .iter()
            .for_each(|perm| assert_eq!("TEST_PERMISSION", perm.as_str()));
    }
}
