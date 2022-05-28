#![doc(html_logo_url = "https://raw.githubusercontent.com/DDtKey/actix-web-grants/main/logo.png")]
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
#![doc = include_str!("../README.md")]

mod guards;
mod middleware;
pub mod permissions;

pub use guards::PermissionGuard;
pub use middleware::GrantsMiddleware;

/// Procedural macros for checking user permissions or roles.
///
/// # Examples
/// ```
/// use actix_web::{web, get, HttpResponse};
/// use actix_web_grants::proc_macro::{has_permissions, has_roles};
/// use actix_web::http::StatusCode;
/// use actix_web::body::BoxBody;
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
///
/// // Custom access denied message.
/// #[has_roles("ADMIN", error = "access_denied()")]
/// async fn role_access() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// // Non-admin role accessor will recive this response.
/// fn access_denied() -> HttpResponse {
///     HttpResponse::with_body(
///         StatusCode::FORBIDDEN,
///         BoxBody::new("This resource allowed only for ADMIN"),
///     )
/// }
///
/// // Additional security condition to ensure the protection of the endpoint
/// #[has_roles("USER", secure = "user_id.into_inner() == user.id")]
/// #[get("/resource/{user_id}")]
/// async fn role_macro_secured_with_params(user_id: web::Path<i32>, user: web::Data<User>) -> HttpResponse {
///     HttpResponse::Ok().body("some secured info with parameters")   
/// }
/// struct User { id: i32 }
///
/// // You own type is also supported (need to configure middleware for this type as well):
/// #[has_roles["Role::Admin", "Role::Manager", type = "Role"]]
/// async fn role_enum_macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// #[derive(PartialEq, Clone)] // required bounds
/// enum Role { Admin, Manager }
///
/// ```
#[cfg(feature = "macro-check")]
pub mod proc_macro {
    pub use actix_grants_proc_macro::*;
}
