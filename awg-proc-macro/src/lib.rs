extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

use crate::expand::HasPermissions;

mod expand;

const HAS_AUTHORITIES: &str = "has_permissions";
const HAS_ANY_AUTHORITY: &str = "has_any_permission";

const HAS_ROLES: &str = "has_roles";
const HAS_ANY_ROLE: &str = "has_any_role";

/// Macro to сheck that the user has all the specified permissions.
/// Allow to add a conditional restriction based on handlers parameters.
/// Add the `secure` attribute followed by the the boolean expression to validate based on parameters
///
/// Also you can use you own types instead of Strings, just add `type` attribute with path to type
/// # Examples
/// ```
/// use actix_web_grants::proc_macro::has_permissions;
/// use actix_web::HttpResponse;
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[has_permissions["ROLE_ADMIN", "OP_GET_SECRET"]]
/// async fn macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
///
/// // User should be ADMIN with OP_GET_SECRET permission and the user.id param should be equal
/// // to the path parameter {user_id}
/// struct User {id: i32}
/// #[has_permissions["ROLE_ADMIN", "OP_GET_SECRET", secure="user_id.into_inner() == user.id"]]
/// async fn macro_secured_params(user_id: web::Path<i32>, user: web::Data<User>) -> HttpResponse {
///     HttpResponse::Ok().body("some secured info with user_id path equal to user.id")
///}
///
/// // User must have MyPermissionEnum::OP_GET_SECRET (you own enum example)
/// #[has_permissions["OP_GET_SECRET", type = "MyPermissionEnum"]]
/// async fn macro_enum_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
///
///```
#[proc_macro_attribute]
pub fn has_permissions(args: TokenStream, input: TokenStream) -> TokenStream {
    check_permissions(HAS_AUTHORITIES, args, input)
}

/// Macro to сheck that the user has any of the specified permissions.
///
/// # Examples
/// ```
/// use actix_web_grants::proc_macro::has_any_permission;
/// use actix_web::HttpResponse;
///
/// // User should be ADMIN or MANAGER
/// #[has_any_permission["ROLE_ADMIN", "ROLE_MANAGER"]]
/// async fn macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// ```
#[proc_macro_attribute]
pub fn has_any_permission(args: TokenStream, input: TokenStream) -> TokenStream {
    check_permissions(HAS_ANY_AUTHORITY, args, input)
}

/// Macro to сheck that the user has all the specified roles.
/// Role - is permission with prefix "ROLE_".
///
/// # Examples
/// ```
/// use actix_web_grants::proc_macro::has_roles;
/// use actix_web::HttpResponse;
///
/// // User should be ADMIN and MANAGER
/// #[has_roles["ADMIN", "MANAGER"]]
/// async fn macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// ```
#[proc_macro_attribute]
pub fn has_roles(args: TokenStream, input: TokenStream) -> TokenStream {
    check_permissions(HAS_ROLES, args, input)
}

/// Macro to сheck that the user has any the specified roles.
/// Role - is permission with prefix "ROLE_".
///
/// # Examples
/// ```
/// use actix_web_grants::proc_macro::has_any_role;
/// use actix_web::HttpResponse;
///
/// // User should be ADMIN or MANAGER
/// #[has_any_role["ADMIN", "MANAGER"]]
/// async fn macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// ```
#[proc_macro_attribute]
pub fn has_any_role(args: TokenStream, input: TokenStream) -> TokenStream {
    check_permissions(HAS_ANY_ROLE, args, input)
}

fn check_permissions(check_fn_name: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let func = parse_macro_input!(input as ItemFn);

    match HasPermissions::new(check_fn_name, args, func) {
        Ok(has_permissions) => has_permissions.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
