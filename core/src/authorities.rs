//! A set of traits and structures to check authorities.

use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;

mod attache;
pub mod extractor;

pub use attache::AttachAuthorities;

/// Trait to check if the user has the required authorities.
pub trait AuthoritiesCheck<T: Eq + Hash> {
    fn has_authority(&self, authority: T) -> bool;
    fn has_authorities(&self, authorities: &[T]) -> bool;
    fn has_any_authority(&self, authorities: &[T]) -> bool;
}

/// Storage for user authorities to keep them as an extension of request.
pub struct AuthDetails<T = String>
where
    T: Eq + Hash,
{
    pub authorities: Arc<HashSet<T>>,
}

impl<T: Eq + Hash> AuthDetails<T> {
    pub fn new(authorities: impl IntoIterator<Item = T>) -> AuthDetails<T> {
        AuthDetails {
            authorities: Arc::new(authorities.into_iter().collect()),
        }
    }
}

impl<T: Eq + Hash> Clone for AuthDetails<T> {
    fn clone(&self) -> Self {
        Self {
            authorities: self.authorities.clone(),
        }
    }
}

impl<T: Eq + Hash> AuthoritiesCheck<&T> for AuthDetails<T> {
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
