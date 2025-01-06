#![doc(
    html_logo_url = "https://raw.githubusercontent.com/DDtKey/protect-endpoints/main/protect-salvo/logo.png"
)]
//! A crate to protect your endpoints in [`salvo`].
//!
//! For built-in configuration, you can use [`GrantsLayer`] tower compatible middleware.
//!
//! To check user access to specific services, you can use [`proc-macro`] or manual.
//!
//! [`permissions`]: authorities
//! [`proc-macro`]: proc_macro
//! [`salvo`]: https://github.com/salvo-rs/salvo
//! [`salvo_extra`]: https://crates.io/crates/salvo_extra
#![doc = include_str!("../README.md")]

use protect_endpoints_core::tower::middleware::GrantsLayer as CoreGrantsLayer;
use salvo::http::ReqBody;

/// Re-export of the `salvo_extra` crate with enabled tower-compatibility.
pub use salvo_extra;

pub mod authorities;

pub type GrantsLayer<Extractor, Type, Err> =
    CoreGrantsLayer<Extractor, salvo::hyper::Request<ReqBody>, Type, Err>;

/// Procedural macros for checking user authorities (permissions or roles).
///
/// # Examples
/// ```
/// use salvo::prelude::*;
/// use serde::{Serialize, Deserialize};
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[protect_salvo::protect("ROLE_ADMIN", "OP_GET_SECRET")]
/// #[handler]
/// async fn macro_secured() -> &'static str {
///     "some secured info"
/// }
///
/// // User should be ADMIN and MANAGER
/// #[protect_salvo::protect("ADMIN", "MANAGER")]
/// #[handler]
/// async fn role_macro_secured() -> &'static str {
///     "some secured info"
/// }
///
/// #[derive(serde::Deserialize, Extractible)]
/// struct UserParams { user_id: i32 }
/// struct User { id: i32 }
///
/// // Additional security condition to ensure the protection of the endpoint.
/// // Compares user_id from the request path with the user.id from the request extensions (might be populated by custom middleware, for example).
/// #[protect_salvo::protect("USER", expr = "Some(params.user_id) == req.extensions().get::<User>().map(|u| u.id)")]
/// #[handler]
/// async fn role_macro_secured_with_params(params: UserParams, req: &mut Request) -> &'static str {
///     "some secured info with parameters"
/// }
///
/// // You own type is also supported (need to configure middleware for this type as well):
/// #[protect_salvo::protect("Role::Admin", "Role::Manager", ty = "Role")]
/// #[handler]
/// async fn role_enum_macro_secured() -> &'static str {
///     "some secured info"
/// }
/// #[derive(Eq, PartialEq, Hash)] // required bounds
/// enum Role { Admin, Manager }
///
/// ```
#[cfg(feature = "macro-check")]
pub mod proc_macro {
    pub use protect_endpoints_proc_macro::protect_salvo as protect;
}

/// Just a shortcut for proc-macros
#[cfg(feature = "macro-check")]
pub use proc_macro::*;
