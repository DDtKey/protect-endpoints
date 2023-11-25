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
/// Also you can use you own types instead of Strings, just add `ty` attribute with path to type
/// # Examples
/// ```
/// use rocket::serde::json::Json;
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[rocket_grants::has_permissions("ROLE_ADMIN", "OP_GET_SECRET")]
/// async fn macro_secured() -> &'static str {
///     "some secured info"
/// }
///
/// // User should be ADMIN with OP_GET_SECRET permission and the user.id param should be equal
/// // to the path parameter {user_id}
/// #[derive(serde::Deserialize)]
/// struct User {id: i32}
///
/// #[rocket_grants::has_permissions("ROLE_ADMIN", "OP_GET_SECRET", secure="user_id == user.id")]
/// async fn macro_secured_params(user_id: i32, user: Json<User>) -> &'static str {
///     "some secured info with user_id path equal to user.id"
///}
///
/// #[derive(Clone, PartialEq, Eq)]
/// enum MyPermissionEnum {
///   OpGetSecret
/// }
///
/// // User must have MyPermissionEnum::OpGetSecret (you own enum example)
/// #[rocket_grants::has_permissions("MyPermissionEnum::OpGetSecret", ty = "MyPermissionEnum")]
/// async fn macro_enum_secured() -> &'static str {
///     "some secured info"
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
/// // User should be ADMIN or MANAGER
/// #[rocket_grants::has_any_permission("ROLE_ADMIN", "ROLE_MANAGER")]
/// async fn macro_secured() -> &'static str {
///     "some secured info"
/// }
/// ```
#[proc_macro_attribute]
pub fn has_any_permission(args: TokenStream, input: TokenStream) -> TokenStream {
    check_permissions(HAS_ANY_AUTHORITY, args, input)
}

/// Macro to сheck that the user has all the specified roles.
/// Role - is permission with prefix "ROLE_" or your own custom type.
///
/// # Examples
/// ```
/// // User should be ADMIN and MANAGER
/// #[rocket_grants::has_roles("ADMIN", "MANAGER")]
/// async fn macro_secured() -> &'static str {
///     "some secured info"
/// }
/// ```
#[proc_macro_attribute]
pub fn has_roles(args: TokenStream, input: TokenStream) -> TokenStream {
    check_permissions(HAS_ROLES, args, input)
}

/// Macro to сheck that the user has any the specified roles.
/// Role - is permission with prefix "ROLE_" or your own custom type.
///
/// # Examples
/// ```
/// // User should be ADMIN or MANAGER
/// #[rocket_grants::has_any_role("ADMIN", "MANAGER")]
/// async fn macro_secured() -> &'static str {
///     "some secured info"
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
