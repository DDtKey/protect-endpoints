//! A set of traits and structures for custom integration.
//!
//! Via [`AuthoritiesExtractor`] implementations, the library gets a user permissions from you.
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

use actix_web::dev::Payload;
use actix_web::error::ErrorUnauthorized;
use actix_web::{Error, FromRequest, HttpMessage, HttpRequest};
use std::future::Future;
use std::pin::Pin;

mod attache;
mod extractors;

pub use attache::AttachAuthorities;
pub use extractors::*;

#[derive(Clone)]
pub struct AuthDetails<T = String>
where
    T: PartialEq,
{
    pub authorities: Vec<T>,
}

impl<T> AuthDetails<T>
where
    T: PartialEq + Clone,
{
    pub fn new(authorities: Vec<T>) -> AuthDetails<T> {
        AuthDetails { authorities }
    }
}

pub trait AuthoritiesCheck<T: PartialEq> {
    fn has_authority(&self, authority: T) -> bool;
    fn has_authorities(&self, authorities: &[T]) -> bool;
    fn has_any_authority(&self, authorities: &[T]) -> bool;
}

impl<T: PartialEq + Clone> AuthoritiesCheck<&T> for AuthDetails<T> {
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
