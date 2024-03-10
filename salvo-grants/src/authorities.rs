use protect_endpoints_core::authorities::AuthDetails as AuthDetailsCore;
use salvo::extract::{Extractible, Metadata};
use salvo::http::StatusCode;
use salvo::{Request, Writer};
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::ops::Deref;

pub use protect_endpoints_core::authorities::AuthoritiesCheck;

pub struct AuthDetails<T = String>(AuthDetailsCore<T>)
where
    T: Eq + Hash;

#[derive(Debug)]
struct AuthDetailsNotFound;

const METADATA: Metadata = Metadata::new("AuthDetails");

impl<'ex, T> Extractible<'ex> for AuthDetails<T>
where
    T: Eq + Hash + Send + Sync + 'static,
{
    fn metadata() -> &'ex Metadata {
        &METADATA
    }

    fn extract(
        req: &'ex mut Request,
    ) -> impl Future<Output = Result<Self, impl Writer + Send + Debug + 'static>> + Send
    where
        Self: Sized,
    {
        async {
            req.extensions()
                .get::<AuthDetailsCore<T>>()
                .cloned()
                .map(AuthDetails)
                .ok_or(AuthDetailsNotFound)
        }
    }
}

#[salvo::async_trait]
impl Writer for AuthDetailsNotFound {
    async fn write(self, _req: &mut Request, _depot: &mut salvo::Depot, res: &mut salvo::Response) {
        res.status_code(StatusCode::UNAUTHORIZED);
    }
}

impl<T: Eq + Hash> Deref for AuthDetails<T> {
    type Target = AuthDetailsCore<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
