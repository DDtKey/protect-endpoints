//! A set of traits and structures for custom integration.
//!
//! See [`GrantsFairing`] for more details.
//!
//! ## If you already have authentication fairing
//! You can integrate it with this library using [`AttachPermissions`]
//!
//!
//! [`AttachPermissions`]: AttachAuthorities
//! [`GrantsFairing`]: rocket_grants::GrantsFairing;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;

mod attache;

pub use attache::AttachAuthorities;

pub struct AuthDetails<T = String> {
    pub authorities: Arc<HashSet<T>>,
}

impl<T> AuthDetails<T>
where
    T: Eq + Hash,
{
    pub fn new(authorities: impl IntoIterator<Item = T>) -> AuthDetails<T> {
        AuthDetails {
            authorities: Arc::new(authorities.into_iter().collect()),
        }
    }
}

pub(crate) struct AuthDetailsWrapper<T>(Option<AuthDetails<T>>);

pub trait AuthoritiesCheck<T: Eq + Hash> {
    fn has_authority(&self, authority: T) -> bool;
    fn has_authorities(&self, authorities: &[T]) -> bool;
    fn has_any_authority(&self, authorities: &[T]) -> bool;
}

impl<T: Eq + Hash + Send + Sync> AuthoritiesCheck<&T> for AuthDetails<T> {
    fn has_authority(&self, authority: &T) -> bool {
        self.authorities.contains(authority)
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
        self.authorities.contains(authority)
    }

    fn has_authorities(&self, authorities: &[&str]) -> bool {
        authorities.iter().all(|auth| self.has_authority(*auth))
    }

    fn has_any_authority(&self, authorities: &[&str]) -> bool {
        authorities.iter().any(|auth| self.has_authority(*auth))
    }
}

#[rocket::async_trait]
impl<'r, T: Eq + Send + Sync + 'static> FromRequest<'r> for AuthDetails<T> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.local_cache(|| AuthDetailsWrapper(None)) {
            AuthDetailsWrapper(Some(details)) => Outcome::Success(details.clone()),
            AuthDetailsWrapper(None) => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

impl<T> Clone for AuthDetails<T> {
    fn clone(&self) -> Self {
        Self {
            authorities: self.authorities.clone(),
        }
    }
}
