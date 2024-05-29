use protect_endpoints_core::authorities::AuthDetails as AuthDetailsCore;
use salvo::extract::{Extractible, Metadata};
use salvo::http::StatusCode;
use salvo::{Request, Response, Writer};
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

pub use protect_endpoints_core::authorities::AuthoritiesCheck;

pub struct AuthDetails<T = String>(AuthDetailsCore<T>)
where
    T: Eq + Hash;

#[derive(Debug)]
pub struct AuthDetailsNotFound;

static METADATA: Metadata = Metadata::new("AuthDetails");

impl<'ex, T> Extractible<'ex> for AuthDetails<T>
where
    T: Eq + Hash + Send + Sync + 'static,
{
    fn metadata() -> &'ex Metadata {
        &METADATA
    }

    async fn extract(req: &'ex mut Request) -> Result<Self, impl Writer + Send + Debug + 'static>
    where
        Self: Sized,
    {
        req.extensions()
            .get::<AuthDetailsCore<T>>()
            .cloned()
            .map(AuthDetails)
            .ok_or(AuthDetailsNotFound)
    }
}

impl salvo::Scribe for AuthDetailsNotFound {
    fn render(self, res: &mut Response) {
        res.status_code(StatusCode::UNAUTHORIZED);
    }
}

impl<T: Eq + Hash> Deref for AuthDetails<T> {
    type Target = AuthDetailsCore<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
