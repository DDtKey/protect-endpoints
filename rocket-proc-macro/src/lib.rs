extern crate proc_macro;
use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};

use crate::expand::{Args, ProtectEndpoint};

mod expand;

/// Macro to сheck that the user has all the specified permissions.
/// Allow to add a conditional restriction based on handlers parameters.
/// Add the `expr` attribute followed by the the boolean expression to validate based on parameters
///
/// Also you can use you own types instead of Strings, just add `ty` attribute with path to type
/// # Examples
/// ```
/// use rocket::serde::json::Json;
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[rocket_grants::protect("ROLE_ADMIN", "OP_GET_SECRET")]
/// async fn macro_secured() -> &'static str {
///     "some secured info"
/// }
///
/// // User should be ADMIN with OP_GET_SECRET permission and the user.id param should be equal
/// // to the path parameter {user_id}
/// #[derive(serde::Deserialize)]
/// struct User {id: i32}
///
/// #[rocket_grants::protect("ROLE_ADMIN", "OP_GET_SECRET", expr="user_id == user.id")]
/// async fn macro_secured_params(user_id: i32, user: Json<User>) -> &'static str {
///     "some secured info with user_id path equal to user.id"
///}
///
/// #[derive(Hash, PartialEq, Eq)]
/// enum MyPermissionEnum {
///   OpGetSecret
/// }
///
/// // User must have MyPermissionEnum::OpGetSecret (you own enum example)
/// #[rocket_grants::protect("MyPermissionEnum::OpGetSecret", ty = MyPermissionEnum)]
/// async fn macro_enum_secured() -> &'static str {
///     "some secured info"
/// }
///
///```
#[proc_macro_attribute]
pub fn protect(args: TokenStream, input: TokenStream) -> TokenStream {
    protect_endpoint(args, input)
}

fn protect_endpoint(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(darling::Error::from(e).write_errors());
        }
    };
    let args = match Args::from_list(&args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let func = parse_macro_input!(input as ItemFn);

    match ProtectEndpoint::new(args, func) {
        Ok(protected) => protected.into_token_stream().into(),
        Err(err) => err.to_compile_error().into(),
    }
}
