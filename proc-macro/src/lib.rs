extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

use crate::expand::HasAuthorities;

mod expand;

const HAS_AUTHORITIES: &str = "has_authorities";
const HAS_ANY_AUTHORITY: &str = "has_any_authority";

const HAS_ROLES: &str = "has_roles";
const HAS_ANY_ROLE: &str = "has_any_role";

/// Macro to сheck that the user has all the specified authorities.
///
/// # Examples
/// ```
/// use actix_web_grants::proc_macro::has_authorities;
/// use actix_web::HttpResponse;
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[has_authorities["ROLE_ADMIN", "OP_GET_SECRET"]]
/// async fn macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// ```
#[proc_macro_attribute]
pub fn has_authorities(args: TokenStream, input: TokenStream) -> TokenStream {
    check_authorities(HAS_AUTHORITIES, args, input)
}


/// Macro to сheck that the user has any of the specified authorities.
///
/// # Examples
/// ```
/// use actix_web_grants::proc_macro::has_any_authority;
/// use actix_web::HttpResponse;
///
/// // User should be ADMIN or MANAGER
/// #[has_any_authority["ROLE_ADMIN", "ROLE_MANAGER"]]
/// async fn macro_secured() -> HttpResponse {
///     HttpResponse::Ok().body("some secured info")
/// }
/// ```
#[proc_macro_attribute]
pub fn has_any_authority(args: TokenStream, input: TokenStream) -> TokenStream {
    check_authorities(HAS_ANY_AUTHORITY, args, input)
}

/// Macro to сheck that the user has all the specified roles.
/// Role - is authority with prefix "ROLE_".
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
    check_authorities(HAS_ROLES, args, input)
}

/// Macro to сheck that the user has any the specified roles.
/// Role - is authority with prefix "ROLE_".
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
    check_authorities(HAS_ANY_ROLE, args, input)
}


fn check_authorities(check_fn_name: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let func = parse_macro_input!(input as ItemFn);

    match HasAuthorities::new(check_fn_name, args, func) {
        Ok(has_authorities) => has_authorities.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
