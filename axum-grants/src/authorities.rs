use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use protect_endpoints_core::authorities::AuthDetails as AuthDetailsCore;
use std::hash::Hash;
use std::ops::Deref;

pub use protect_endpoints_core::authorities::AuthoritiesCheck;

pub struct AuthDetails<T = String>(AuthDetailsCore<T>)
where
    T: Eq + Hash;

#[axum::async_trait]
impl<S, T> FromRequestParts<S> for AuthDetails<T>
where
    T: Eq + Hash + Send + Sync + 'static,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthDetailsCore<T>>()
            .cloned()
            .map(AuthDetails)
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)
    }
}

impl<T: Eq + Hash> Deref for AuthDetails<T> {
    type Target = AuthDetailsCore<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
