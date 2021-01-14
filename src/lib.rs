//! A crate for validate user authoritites in [`actix-web`].
//!
//! For built-in configure see: [`GrantsMiddleware`].
//!
//! The library can also be integrated with third-party solutions (like [`httpauth`]), see [`authorities`] module.
//!
//! You can find more examples in the git repository, `examples` folder.
//!
//! [`actix-web`]: actix_web
//! [`GrantsMiddleware`]: GrantsMiddleware
//! [`httpauth`]: https://docs.rs/actix-web-httpauth
//! [`authorities`]: authorities


pub mod authorities;
mod middleware;

pub use middleware::GrantsMiddleware;


/// Procedural macros for checking user authorities or roles.

/// # Examples
/// ```
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
pub mod proc_macro {
    pub use actix_grants_proc_macro::*;
}
