//! A set of traits and structures for custom integration.
//!
//! Via [`PermissionsExtractor`] implementations, the library gets a user permissions from you.
//! The default implementation of the [`PermissionsExtractor`] is provided via a function.
//!
//! See [`GrantsMiddleware`] for more details.
//!
//! ## If you already have authentication middleware
//! You can integrate it with this library using [`AttachPermissions`]
//!
//!
//! [`PermissionsExtractor`]: PermissionsExtractor
//! [`AttachPermissions`]: AttachPermissions
//! [`GrantsMiddleware`]: poem_grants::GrantsMiddleware;

use poem::{FromRequest, Request, RequestBody};

mod attache;
mod extractors;

use crate::error::AccessError;
pub use attache::AttachPermissions;
pub use extractors::*;

#[derive(Clone)]
pub struct AuthDetails<T = String>
where
    T: PartialEq + Send + Sync,
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

#[poem::async_trait]
impl<'a, T: PartialEq + Clone + Send + Sync + 'static> FromRequest<'a> for AuthDetails<T> {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> poem::Result<Self> {
        req.extensions()
            .get::<AuthDetails<T>>()
            .map(AuthDetails::clone)
            .ok_or(AccessError::UnauthorizedRequest)
            .map_err(Into::into)
    }
}
