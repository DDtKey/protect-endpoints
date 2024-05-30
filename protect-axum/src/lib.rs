#![doc(
    html_logo_url = "https://raw.githubusercontent.com/DDtKey/protect-endpoints/main/protect-axum/logo.png"
)]
//! A crate to protect your endpoints in [`axum`].
//!
//! For built-in configuration, you can use [`GrantsLayer`] tower compatible middleware.
//!
//! To check user access to specific services, you can use [`proc-macro`] or manual.
//!
//! [`permissions`]: authorities
//! [`proc-macro`]: proc_macro
//! [`axum`]: https://github.com/tokio-rs/axum
#![doc = include_str!("../README.md")]

use protect_endpoints_core::tower::middleware::GrantsLayer as CoreGrantsLayer;

pub mod authorities;

pub type GrantsLayer<Extractor, Type, Err> =
    CoreGrantsLayer<Extractor, axum::extract::Request, Type, Err>;

/// Procedural macros for checking user authorities (permissions or roles).
///
/// # Examples
/// ```
/// use axum::{http::StatusCode, Extension};
/// use axum::extract::Path;
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[protect_axum::protect("ROLE_ADMIN", "OP_GET_SECRET")]
/// async fn macro_secured() -> (StatusCode, &'static str) {
///     (StatusCode::OK, "some secured info")
/// }
///
/// // User should be ADMIN and MANAGER
/// #[protect_axum::protect("ADMIN", "MANAGER")]
/// async fn role_macro_secured() -> (StatusCode, &'static str) {
///     (StatusCode::OK, "some secured info")
/// }
///
/// // Additional security condition to ensure the protection of the endpoint
/// #[protect_axum::protect("USER", expr = "*user_id == user.id")]
/// async fn role_macro_secured_with_params(user_id: Path<i32>, user: Extension<&User>) -> (StatusCode, &'static str) {
///     (StatusCode::OK, "some secured info with parameters")
/// }
/// struct User { id: i32 }
///
/// // You own type is also supported (need to configure middleware for this type as well):
/// #[protect_axum::protect("Role::Admin", "Role::Manager", ty = "Role")]
/// async fn role_enum_macro_secured() -> (StatusCode, &'static str) {
///     (StatusCode::OK, "some secured info")
/// }
/// #[derive(Eq, PartialEq, Hash)] // required bounds
/// enum Role { Admin, Manager }
///
/// ```
#[cfg(feature = "macro-check")]
pub mod proc_macro {
    pub use protect_endpoints_proc_macro::protect_axum as protect;
}

/// Just a shortcut for proc-macros
#[cfg(feature = "macro-check")]
pub use proc_macro::*;
