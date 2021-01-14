//! A crate for validate user authoritites in [`actix-web`].
//!
//! For built-in configure see: [`GrantsMiddleware`].
//!
//! To check user access to specific services, you can use: [`proc-macro`] and [`AuthorityGuard`] or manual.
//!
//! The library can also be integrated with third-party solutions (like [`httpauth`]), see [`authorities`] module.
//!
//! You can find more [`examples`] in the git repository.
//!
//! [`actix-web`]: actix_web
//! [`GrantsMiddleware`]: GrantsMiddleware
//! [`httpauth`]: https://docs.rs/actix-web-httpauth
//! [`examples`]: https://github.com/DDtKey/actix-web-grants/tree/main/examples
//! [`authorities`]: authorities
//! [`proc-macro`]: proc_macro
//! [`AuthorityGuard`]: AuthorityGuard

pub mod authorities;
mod guards;
mod middleware;

pub use guards::AuthorityGuard;
pub use middleware::GrantsMiddleware;

/// Procedural macros for checking user authorities or roles.
///
/// # Examples
/// ```
/// use actix_web::{HttpResponse};
/// use actix_web_grants::proc_macro::{has_authorities, has_roles};
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[has_authorities["ROLE_ADMIN", "OP_GET_SECRET"]]
/// async fn macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
///
/// // Role - is authority with prefix "ROLE_".
/// // User should be ADMIN and MANAGER
/// #[has_roles["ADMIN", "MANAGER"]]
/// async fn role_macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// ```
#[cfg(feature = "macro-check")]
pub mod proc_macro {
    pub use actix_grants_proc_macro::*;
}
