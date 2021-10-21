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

use actix_web::dev::{Payload, PayloadStream};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;

mod attache;
mod extractors;

pub use attache::AttachPermissions;
pub use extractors::*;

#[derive(Clone)]
pub struct AuthDetails {
    pub permissions: Vec<String>,
}

impl AuthDetails {
    pub fn new(permissions: Vec<String>) -> AuthDetails {
        AuthDetails { permissions }
    }
}

pub trait PermissionsCheck {
    fn has_permission(&self, permission: &str) -> bool;
    fn has_permissions(&self, permissions: Vec<&str>) -> bool;
    fn has_any_permission(&self, permissions: Vec<&str>) -> bool;
}

impl PermissionsCheck for AuthDetails {
    fn has_permission(&self, permission: &str) -> bool {
        self.permissions
            .iter()
            .any(|auth| auth.as_str() == permission)
    }

    fn has_permissions(&self, permissions: Vec<&str>) -> bool {
        permissions
            .into_iter()
            .all(|auth| self.has_permission(auth))
    }

    fn has_any_permission(&self, permissions: Vec<&str>) -> bool {
        permissions
            .into_iter()
            .any(|auth| self.has_permission(auth))
    }
}

pub(crate) const ROLE_PREFIX: &str = "ROLE_";

pub trait RolesCheck {
    fn has_role(&self, permission: &str) -> bool;
    fn has_roles(&self, permissions: Vec<&str>) -> bool;
    fn has_any_role(&self, permissions: Vec<&str>) -> bool;
}

impl RolesCheck for AuthDetails {
    fn has_role(&self, permission: &str) -> bool {
        let permission = format!("{}{}", ROLE_PREFIX, permission);

        self.permissions.iter().any(|auth| auth == &permission)
    }

    fn has_roles(&self, permissions: Vec<&str>) -> bool {
        permissions.into_iter().all(|auth| self.has_role(auth))
    }

    fn has_any_role(&self, permissions: Vec<&str>) -> bool {
        permissions.into_iter().any(|auth| self.has_role(auth))
    }
}

impl FromRequest for AuthDetails {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload<PayloadStream>) -> Self::Future {
        let req = req.clone();

        Box::pin(async move {
            req.extensions()
                .get::<AuthDetails>()
                .map(AuthDetails::clone)
                .ok_or_else(|| ErrorUnauthorized("User unauthorized!"))
        })
    }
}
