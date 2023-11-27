#![doc(
    html_logo_url = "https://raw.githubusercontent.com/DDtKey/protect-endpoints/main/actix-web-grants/logo.png"
)]
//! A crate to protect your endpoints in `actix-web`.
//!
//! For built-in configure see: [`GrantsMiddleware`].
//!
//! To check user access to specific services, you can use: [`proc-macro`] and [`AuthorityGuard`] or manual.
//!
//! The library can also be integrated with third-party solutions (like [`httpauth`]), see [`authorities`] module.
//!
//! You can find more [`examples`] in the git repository.
//!
//! [`GrantsMiddleware`]: GrantsMiddleware
//! [`httpauth`]: https://docs.rs/actix-web-httpauth
//! [`examples`]: https://github.com/DDtKey/protect-endpoints/tree/main/examples/actix-web
//! [`authorities`]: authorities
//! [`proc-macro`]: proc_macro
//! [`AuthorityGuard`]: AuthorityGuard
#![doc = include_str!("../README.md")]

pub mod authorities;
mod guards;
mod middleware;

pub use guards::AuthorityGuard;
pub use middleware::GrantsMiddleware;

/// Procedural macros for checking user authorities (permissions or roles).
///
/// # Examples
/// ```
/// use actix_web::{web, get, HttpResponse};
/// use actix_web_grants::protect;
/// use actix_web::http::StatusCode;
/// use actix_web::body::BoxBody;
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[protect("ROLE_ADMIN", "OP_GET_SECRET")]
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
///
/// // Custom access denied message.
/// #[protect("ADMIN", error = "access_denied")]
/// async fn role_access() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// // Non-admin role accessor will receive this response.
/// // The return type of the custom function must be `actix web::HttpResponse`.
/// fn access_denied() -> HttpResponse {
///     HttpResponse::with_body(
///         StatusCode::FORBIDDEN,
///         BoxBody::new("This resource allowed only for ADMIN"),
///     )
/// }
///
/// // Additional security condition to ensure the protection of the endpoint
/// #[protect("USER", expr = "user_id.into_inner() == user.id")]
/// #[get("/resource/{user_id}")]
/// async fn role_macro_secured_with_params(user_id: web::Path<i32>, user: web::Data<User>) -> HttpResponse {
///     HttpResponse::Ok().body("some secured info with parameters")   
/// }
/// struct User { id: i32 }
///
/// // You own type is also supported (need to configure middleware for this type as well):
/// #[protect("Role::Admin", "Role::Manager", ty = "Role")]
/// async fn role_enum_macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// #[derive(PartialEq, Clone)] // required bounds
/// enum Role { Admin, Manager }
///
/// ```
#[cfg(feature = "macro-check")]
pub mod proc_macro {
    pub use protect_endpoints_proc_macro::protect_actix_web as protect;
}

/// Just a shortcut for proc-macros
#[cfg(feature = "macro-check")]
pub use proc_macro::*;
