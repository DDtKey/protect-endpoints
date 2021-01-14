//! A set of traits and structures for custom integration.
//!
//! Via [`AuthoritiesExtractor`] implementations, the library gets a user authoritites from you.
//! The default implementation of the [`AuthoritiesExtractor`] is provided via a function.
//!
//! See [`GrantsMiddleware`] for more details.
//!
//! ## If you already have middleware authorization
//! You can integrate it with this library using [`AttachAuthorities`]
//!
//!
//! [`AuthoritiesExtractor`]: AuthoritiesExtractor
//! [`AttachAuthorities`]: AttachAuthorities
//! [`GrantsMiddleware`]: actix_web_grants::GrantsMiddleware;

use actix_web::dev::{Payload, PayloadStream};
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;

mod attache;
mod extractors;

pub use attache::AttachAuthorities;
pub use extractors::*;

#[derive(Clone)]
pub struct AuthDetails {
    pub authorities: Vec<String>,
}

impl AuthDetails {
    pub fn new(authorities: Vec<String>) -> AuthDetails {
        AuthDetails { authorities }
    }
}

pub trait AuthoritiesCheck {
    fn has_authority(&self, authority: &str) -> bool;
    fn has_authorities(&self, authorities: Vec<&str>) -> bool;
    fn has_any_authority(&self, authorities: Vec<&str>) -> bool;
}

impl AuthoritiesCheck for AuthDetails {
    fn has_authority(&self, authority: &str) -> bool {
        self.authorities
            .iter()
            .any(|auth| auth.as_str() == authority)
    }

    fn has_authorities(&self, authorities: Vec<&str>) -> bool {
        authorities.into_iter().all(|auth| self.has_authority(auth))
    }

    fn has_any_authority(&self, authorities: Vec<&str>) -> bool {
        authorities.into_iter().any(|auth| self.has_authority(auth))
    }
}

pub(crate) const ROLE_PREFIX: &str = "ROLE_";

pub trait RolesCheck {
    fn has_role(&self, authority: &str) -> bool;
    fn has_roles(&self, authorities: Vec<&str>) -> bool;
    fn has_any_role(&self, authorities: Vec<&str>) -> bool;
}

impl RolesCheck for AuthDetails {
    fn has_role(&self, authority: &str) -> bool {
        let authority = format!("{}{}", ROLE_PREFIX, authority);

        self.authorities.iter().any(|auth| auth == &authority)
    }

    fn has_roles(&self, authorities: Vec<&str>) -> bool {
        authorities.into_iter().all(|auth| self.has_role(auth))
    }

    fn has_any_role(&self, authorities: Vec<&str>) -> bool {
        authorities.into_iter().any(|auth| self.has_role(auth))
    }
}

impl FromRequest for AuthDetails {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Error>>>>;
    type Config = ();

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
