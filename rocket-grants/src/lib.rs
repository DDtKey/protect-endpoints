#![doc(html_logo_url = "https://raw.githubusercontent.com/DDtKey/rocket-grants/main/logo.png")]
//! A crate for authorization in `rocket`.
//!
//! For built-in configure see: [`GrantsFairing`].
//!
//! To check user access to specific services, you can use [`proc-macro`] or manual.
//!
//! The library can also be integrated with third-party solutions or your custom fairings, see [`permissions`] module.
//!
//! You can find more [`examples`] in the git repository.
//!
//! [`GrantsFairing`]: GrantsFairing
//! [`examples`]: https://github.com/DDtKey/rocket-grants/tree/main/examples
//! [`permissions`]: permissions
//! [`proc-macro`]: proc_macro
#![doc = include_str!("../README.md")]

mod fairing;
pub mod permissions;

pub use fairing::GrantsFairing;

/// Procedural macros for checking user permissions or roles.
///
/// # Examples
/// ```
/// use rocket::{Response, http::Status};
/// use rocket::serde::json::Json;
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[rocket_grants::protect("ROLE_ADMIN", "OP_GET_SECRET")]
/// #[rocket::get("/")]
/// async fn macro_secured() -> &'static str {
///    "some secured info"
/// }
///
/// // Role - is string with prefix "ROLE_".
/// // User should be ADMIN and MANAGER
/// #[rocket_grants::protect("ADMIN", "MANAGER")]
/// #[rocket::get("/role")]
/// async fn role_macro_secured() -> &'static str {
///    "some secured info"
/// }
///
/// // Additional security condition to ensure the protection of the endpoint
/// #[rocket_grants::protect("USER", secure = "user_id == user.id")]
/// #[rocket::post("/secure/<user_id>", data = "<user>")]
/// async fn role_macro_secured_with_params(user_id: i32, user: Json<User>) -> &'static str {
///    "some secured info with parameters"
/// }
///
/// #[derive(serde::Deserialize)]
/// struct User { id: i32 }
///
/// // You own type is also supported (need to configure fairing for this type as well):
/// #[rocket_grants::protect(any("Role::Admin", "Role::Manager"), ty = Role)]
/// #[rocket::get("/enum")]
/// async fn role_enum_macro_secured() -> &'static str {
///    "some secured info"
/// }
/// #[derive(PartialEq, Clone)] // required bounds
/// enum Role { Admin, Manager }
///
/// ```

#[cfg(feature = "macro-check")]
pub mod proc_macro {
    pub use rocket_grants_proc_macro::*;
}

/// Just a shortcut for proc-macros
#[cfg(feature = "macro-check")]
pub use proc_macro::*;
