//! A set of traits and structures for custom integration.
//!
//! Via [`PermissionsExtractor`] implementations, the library gets a user permissions from you.
//! The default implementation of the [`PermissionsExtractor`] is provided via a function.
//!
//! See [`GrantsMiddleware`] for more details.
//!
//! ## If you already have middleware authorization
//! You can integrate it with this library using [`AttachPermissions`]
//!
//!
//! [`PermissionsExtractor`]: PermissionsExtractor
//! [`AttachPermissions`]: AttachPermissions
//! [`GrantsMiddleware`]: actix_web_grants::GrantsMiddleware;

use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use std::future::Future;
use std::pin::Pin;

mod attache;
mod extractors;

pub use attache::AttachPermissions;
pub use extractors::*;

#[derive(Clone)]
pub struct AuthDetails<T = String>
where
    T: PartialEq,
{
    pub permissions: Vec<T>,
}

impl<T> AuthDetails<T>
where
    T: PartialEq + Clone,
{
    pub fn new(permissions: Vec<T>) -> AuthDetails<T> {
        AuthDetails { permissions }
    }
}

pub trait PermissionsCheck<T: PartialEq> {
    fn has_permission(&self, permission: T) -> bool;
    fn has_permissions(&self, permissions: &[T]) -> bool;
    fn has_any_permission(&self, permissions: &[T]) -> bool;
}

impl<T: PartialEq + Clone> PermissionsCheck<&T> for AuthDetails<T> {
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

impl<T: PartialEq + Clone> RolesCheck<&T> for AuthDetails<T> {
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

impl<T: PartialEq + Clone + 'static> FromRequest for AuthDetails<T> {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            req.extensions()
                .get::<AuthDetails<T>>()
                .map(AuthDetails::clone)
                .ok_or_else(|| ErrorUnauthorized("User unauthorized!"))
        })
    }
}
