//! A set of traits and structures for custom integration.
//!
//! See [`GrantsFairing`] for more details.
//!
//! ## If you already have authentication fairing
//! You can integrate it with this library using [`AttachPermissions`]
//!
//!
//! [`AttachPermissions`]: AttachPermissions
//! [`GrantsFairing`]: rocket_grants::GrantsFairing;

use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use std::collections::HashSet;
use std::hash::Hash;
use std::sync::Arc;

mod attache;

pub use attache::AttachPermissions;

pub struct AuthDetails<T = String> {
    pub permissions: Arc<HashSet<T>>,
}

impl<T> AuthDetails<T>
where
    T: Eq + Hash,
{
    pub fn new(permissions: impl IntoIterator<Item = T>) -> AuthDetails<T> {
        AuthDetails {
            permissions: Arc::new(permissions.into_iter().collect()),
        }
    }
}

pub(crate) struct AuthDetailsWrapper<T: Eq>(Option<AuthDetails<T>>);

pub trait PermissionsCheck<T: Eq + Hash> {
    fn has_permission(&self, permission: T) -> bool;
    fn has_permissions(&self, permissions: &[T]) -> bool;
    fn has_any_permission(&self, permissions: &[T]) -> bool;
}

impl<T: Eq + Hash + Send + Sync> PermissionsCheck<&T> for AuthDetails<T> {
    fn has_permission(&self, permission: &T) -> bool {
        self.permissions.contains(permission)
    }

    fn has_permissions(&self, permissions: &[&T]) -> bool {
        permissions.iter().all(|auth| self.has_permission(auth))
    }

    fn has_any_permission(&self, permissions: &[&T]) -> bool {
        permissions.iter().any(|auth| self.has_permission(auth))
    }
}

impl PermissionsCheck<&str> for AuthDetails {
    fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(permission)
    }

    fn has_permissions(&self, permissions: &[&str]) -> bool {
        permissions.iter().all(|auth| self.has_permission(*auth))
    }

    fn has_any_permission(&self, permissions: &[&str]) -> bool {
        permissions.iter().any(|auth| self.has_permission(*auth))
    }
}

pub trait RolesCheck<T> {
    fn has_role(&self, permission: T) -> bool;
    fn has_roles(&self, permissions: &[T]) -> bool;
    fn has_any_role(&self, permissions: &[T]) -> bool;
}

pub(crate) const ROLE_PREFIX: &str = "ROLE_";

impl RolesCheck<&str> for AuthDetails {
    fn has_role(&self, permission: &str) -> bool {
        let permission = format!("{}{}", ROLE_PREFIX, permission);

        self.permissions.contains(&permission)
    }

    fn has_roles(&self, permissions: &[&str]) -> bool {
        permissions.iter().all(|auth| self.has_role(*auth))
    }

    fn has_any_role(&self, permissions: &[&str]) -> bool {
        permissions.iter().any(|auth| self.has_role(*auth))
    }
}

impl<T: Eq + Hash + Send + Sync> RolesCheck<&T> for AuthDetails<T> {
    fn has_role(&self, permission: &T) -> bool {
        self.permissions.contains(permission)
    }

    fn has_roles(&self, permissions: &[&T]) -> bool {
        permissions.iter().all(|auth| self.has_role(auth))
    }

    fn has_any_role(&self, permissions: &[&T]) -> bool {
        permissions.iter().any(|auth| self.has_role(auth))
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
            permissions: self.permissions.clone(),
        }
    }
}
