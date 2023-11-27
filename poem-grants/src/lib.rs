#![doc(
    html_logo_url = "https://raw.githubusercontent.com/DDtKey/protect-endpoints/main/poem-grants/logo.png"
)]
//! A crate to protect your endpoints in `poem`.
//!
//! For built-in configure see: [`GrantsMiddleware`].
//!
//! To check user access to specific services, you can use [`proc-macro`] or manual.
//!
//! The library can also be integrated with third-party solutions or your custom middlewares, see [`authorities`] module.
//!
//! You can find more [`examples`] in the git repository.
//!
//! [`GrantsMiddleware`]: GrantsMiddleware
//! [`examples`]: https://github.com/DDtKey/protect-endpoints/tree/main/examples/poem
//! [`authorities`]: authorities
//! [`proc-macro`]: proc_macro
#![doc = include_str!("../README.md")]

pub mod authorities;
pub mod error;
mod middleware;

pub use middleware::GrantsMiddleware;

/// Procedural macros for checking user authorities (permissions or roles).
///
/// # Examples
/// ```
/// use poem::{Response, http::StatusCode, web};
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[poem_grants::protect("ROLE_ADMIN", "OP_GET_SECRET")]
/// #[poem::handler]
/// async fn macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
/// }
///
/// // User should be ADMIN and MANAGER
/// #[poem_grants::protect("ADMIN", "MANAGER")]
/// #[poem::handler]
/// async fn role_macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
/// }
///
/// // Additional security condition to ensure the protection of the endpoint
/// #[poem_grants::protect("USER", expr = "*user_id == user.id")]
/// #[poem::handler]
/// async fn role_macro_secured_with_params(user_id: web::Path<i32>, user: web::Data<&User>) -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info with parameters")
/// }
/// struct User { id: i32 }
///
/// // You own type is also supported (need to configure middleware for this type as well):
/// #[poem_grants::protect("Role::Admin", "Role::Manager", ty = "Role")]
/// #[poem::handler]
/// async fn role_enum_macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
/// }
/// #[derive(PartialEq, Clone)] // required bounds
/// enum Role { Admin, Manager }
///
/// ```
#[cfg(feature = "macro-check")]
pub mod proc_macro {
    pub use protect_endpoints_proc_macro::{open_api, protect_poem as protect};
}

/// Just a shortcut for proc-macros
#[cfg(feature = "macro-check")]
pub use proc_macro::*;
