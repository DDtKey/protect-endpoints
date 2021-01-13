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

mod attache;

pub use attache::AttachPermissions;

#[derive(Clone)]
pub struct AuthDetails<T = String>
where
    T: PartialEq + Clone + Send + Sync,
{
    pub permissions: Vec<T>,
}

impl<T> AuthDetails<T>
where
    T: PartialEq + Clone + Send + Sync,
{
    pub fn new(permissions: Vec<T>) -> AuthDetails<T> {
        AuthDetails { permissions }
    }
}

pub(crate) struct AuthDetailsWrapper<T: PartialEq + Clone + Send + Sync>(Option<AuthDetails<T>>);

pub trait PermissionsCheck<T: PartialEq + Send + Sync> {
    fn has_permission(&self, permission: T) -> bool;
    fn has_permissions(&self, permissions: &[T]) -> bool;
    fn has_any_permission(&self, permissions: &[T]) -> bool;
}

impl<T: PartialEq + Clone + Send + Sync> PermissionsCheck<&T> for AuthDetails<T> {
    fn has_permission(&self, permission: &T) -> bool {
        self.permissions.iter().any(|auth| auth == permission)
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
        self.permissions
            .iter()
            .any(|auth| auth.as_str() == permission)
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

        self.permissions.iter().any(|auth| auth == &permission)
    }

    fn has_roles(&self, permissions: &[&str]) -> bool {
        permissions.iter().all(|auth| self.has_role(*auth))
    }

    fn has_any_role(&self, permissions: &[&str]) -> bool {
        permissions.iter().any(|auth| self.has_role(*auth))
    }
}

impl<T: PartialEq + Clone + Send + Sync> RolesCheck<&T> for AuthDetails<T> {
    fn has_role(&self, permission: &T) -> bool {
        self.permissions.iter().any(|auth| auth == permission)
    }

    fn has_roles(&self, permissions: &[&T]) -> bool {
        permissions.iter().all(|auth| self.has_role(auth))
    }

    fn has_any_role(&self, permissions: &[&T]) -> bool {
        permissions.iter().any(|auth| self.has_role(auth))
    }
}

#[rocket::async_trait]
impl<'r, T: PartialEq + Clone + Send + Sync + 'static> FromRequest<'r> for AuthDetails<T> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.local_cache(|| AuthDetailsWrapper(None)) {
            AuthDetailsWrapper(Some(details)) => Outcome::Success(details.clone()),
            AuthDetailsWrapper(None) => Outcome::Failure((Status::Unauthorized, ())),
        }
    }
}
