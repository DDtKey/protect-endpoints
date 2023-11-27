//! A set of traits and structures for custom integration.
//!
//! Via [`AuthoritiesExtractor`] implementations, the library gets a user authorities (permissions or roles) from you.
//! The default implementation of the [`AuthoritiesExtractor`] is provided via a function.
//!
//! See [`GrantsMiddleware`] for more details.
//!
//! ## If you already have authentication middleware
//! You can integrate it with this library using [`AttachAuthorities`]
//!
//!
//! [`AuthoritiesExtractor`]: AuthoritiesExtractor
//! [`AttachAuthorities`]: AttachAuthorities
//! [`GrantsMiddleware`]: poem_grants::GrantsMiddleware;

use poem::{FromRequest, Request, RequestBody};
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;

mod attache;
mod extractors;

use crate::error::AccessError;
pub use attache::AttachAuthorities;
pub use extractors::*;

pub struct AuthDetails<T = String> {
    pub authorities: Arc<HashSet<T>>,
}

impl<T> AuthDetails<T>
where
    T: Eq + Hash + Send + Sync,
{
    pub fn new(authorities: impl IntoIterator<Item = T>) -> AuthDetails<T> {
        AuthDetails {
            authorities: Arc::new(authorities.into_iter().collect()),
        }
    }
}

pub trait AuthoritiesCheck<T: Eq + Hash + Send + Sync> {
    fn has_authority(&self, authority: T) -> bool;
    fn has_authorities(&self, authorities: &[T]) -> bool;
    fn has_any_authority(&self, authorities: &[T]) -> bool;
}

impl<T: Eq + Hash + Send + Sync> AuthoritiesCheck<&T> for AuthDetails<T> {
    fn has_authority(&self, authority: &T) -> bool {
        self.authorities.iter().any(|auth| auth == authority)
    }

    fn has_authorities(&self, authorities: &[&T]) -> bool {
        authorities.iter().all(|auth| self.has_authority(auth))
    }

    fn has_any_authority(&self, authorities: &[&T]) -> bool {
        authorities.iter().any(|auth| self.has_authority(auth))
    }
}

impl AuthoritiesCheck<&str> for AuthDetails {
    fn has_authority(&self, authority: &str) -> bool {
        self.authorities
            .iter()
            .any(|auth| auth.as_str() == authority)
    }

    fn has_authorities(&self, authorities: &[&str]) -> bool {
        authorities.iter().all(|auth| self.has_authority(*auth))
    }

    fn has_any_authority(&self, authorities: &[&str]) -> bool {
        authorities.iter().any(|auth| self.has_authority(*auth))
    }
}

#[poem::async_trait]
impl<'a, T: Eq + Hash + Send + Sync + 'static> FromRequest<'a> for AuthDetails<T> {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        req.extensions()
            .get::<AuthDetails<T>>()
            .map(AuthDetails::clone)
            .ok_or(AccessError::UnauthorizedRequest)
            .map_err(Into::into)
    }
}

impl<T: Eq + Hash> Clone for AuthDetails<T> {
    fn clone(&self) -> Self {
        Self {
            authorities: self.authorities.clone(),
        }
    }
}
