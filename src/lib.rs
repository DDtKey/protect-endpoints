//! A crate for validate user permissions in `actix-web`.
//!
//! For built-in configure see: [`GrantsMiddleware`].
//!
//! To check user access to specific services, you can use: [`proc-macro`] and [`PermissionGuard`] or manual.
//!
//! The library can also be integrated with third-party solutions (like [`httpauth`]), see [`permissions`] module.
//!
//! You can find more [`examples`] in the git repository.
//!
//! [`GrantsMiddleware`]: GrantsMiddleware
//! [`httpauth`]: https://docs.rs/actix-web-httpauth
//! [`examples`]: https://github.com/DDtKey/actix-web-grants/tree/main/examples
//! [`permissions`]: permissions
//! [`proc-macro`]: proc_macro
//! [`PermissionGuard`]: PermissionGuard

mod guards;
mod middleware;
pub mod permissions;

pub use guards::PermissionGuard;
pub use middleware::GrantsMiddleware;

/// Procedural macros for checking user permissions or roles.
///
/// # Examples
/// ```
/// use actix_web::{HttpResponse};
/// use actix_web_grants::proc_macro::{has_permissions, has_roles};
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[has_permissions["ROLE_ADMIN", "OP_GET_SECRET"]]
/// async fn macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
///
/// // Role - is permission with prefix "ROLE_".
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
