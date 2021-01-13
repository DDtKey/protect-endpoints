#![doc(html_logo_url = "https://raw.githubusercontent.com/DDtKey/poem-grants/main/logo.png")]
//! A crate for authorization in `poem`.
//!
//! For built-in configure see: [`GrantsMiddleware`].
//!
//! To check user access to specific services, you can use [`proc-macro`] or manual.
//!
//! The library can also be integrated with third-party solutions or your cusom middlewares, see [`permissions`] module.
//!
//! You can find more [`examples`] in the git repository.
//!
//! [`GrantsMiddleware`]: GrantsMiddleware
//! [`examples`]: https://github.com/DDtKey/poem-grants/tree/main/examples
//! [`permissions`]: permissions
//! [`proc-macro`]: proc_macro
#![doc = include_str!("../README.md")]

pub mod error;
mod middleware;
pub mod permissions;

pub use middleware::GrantsMiddleware;

/// Procedural macros for checking user permissions or roles.
///
/// # Examples
/// ```
/// use poem::{Response, http::StatusCode, web};
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[poem_grants::has_permissions["ROLE_ADMIN", "OP_GET_SECRET"]]
/// async fn macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
/// }
///
/// // Role - is permission with prefix "ROLE_".
/// // User should be ADMIN and MANAGER
/// #[poem_grants::has_roles["ADMIN", "MANAGER"]]
/// async fn role_macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
/// }
///
/// // Additional security condition to ensure the protection of the endpoint
/// #[poem_grants::has_roles("USER", secure = "*user_id == user.id")]
/// async fn role_macro_secured_with_params(user_id: web::Path<i32>, user: web::Data<&User>) -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info with parameters")
/// }
/// struct User { id: i32 }
///
/// // You own type is also supported (need to configure middleware for this type as well):
/// #[poem_grants::has_roles["Role::Admin", "Role::Manager", type = "Role"]]
/// async fn role_enum_macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
/// }
/// #[derive(PartialEq, Clone)] // required bounds
/// enum Role { Admin, Manager }
///
/// ```
#[cfg(feature = "macro-check")]
pub mod proc_macro {
    pub use poem_grants_proc_macro::*;
}

/// Just a shortcut for proc-macros
#[cfg(feature = "macro-check")]
pub use proc_macro::*;
