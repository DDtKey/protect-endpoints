extern crate proc_macro;
use darling::ast::NestedMeta;
use darling::FromMeta;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, ItemFn};

use crate::expand::{Args, FnType, ProtectEndpoint};

mod expand;

/// Macro to сheck that the user has all the specified permissions.
/// Allow to add a conditional restriction based on handlers parameters.
/// Add the `expr` attribute followed by the the boolean expression to validate based on parameters
///
/// Also you can use you own types instead of Strings, just add `ty` attribute with path to type
/// # Examples
/// ```rust,no_run
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[protect("ROLE_ADMIN", "OP_GET_SECRET")]
/// async fn macro_secured() -> &'static str {
///     "some secured info"
/// }
///
/// // User should be ADMIN with OP_GET_SECRET permission and the user.id param should be equal
/// // to the path parameter {user_id}
/// #[derive(serde::Deserialize)]
/// struct User {id: i32}
///
/// #[protect("ROLE_ADMIN", "OP_GET_SECRET", expr="user_id == user.id")]
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
/// #[protect("MyPermissionEnum::OpGetSecret", ty = MyPermissionEnum)]
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

    ProtectEndpoint::new(args, FnType::Fn(func))
        .into_token_stream()
        .into()
}

/// Macro for `poem-openapi` support
/// Add macro `#[poem_grants::open_api]` above of `#[poem_openapi::OpenApi]` and mark all needed methods with necessary security-methods:
/// One of [`has_permissions`, `has_any_permission`, `has_roles`, `has_any_role`]
///
/// # Examples
/// ```rust,no_run
/// struct Api;
///
/// #[poem_grants::open_api]
/// #[poem_openapi::OpenApi]
/// impl Api {
///     // An example of protection via `proc-macro`.
///     // Just use the necessary name of macro provided by `poem-grants` without crate-name:
///     #[has_permissions("OP_READ_ADMIN_INFO")]
///     #[oai(path = "/admin", method = "get")]
///     async fn macro_secured(&self) -> PlainText<String> {
///         PlainText("ADMIN_RESPONSE".to_string())
///     }
/// }
/// ```
#[cfg(feature = "poem")]
#[cfg_attr(docsrs, doc(cfg(feature = "poem")))]
#[proc_macro_attribute]
pub fn open_api(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(input as syn::ItemImpl);
    let mut methods = Vec::new();
    for (idx, item) in item_impl.items.iter().enumerate() {
        if let syn::ImplItem::Fn(method) = item {
            for grants_attr in method
                .attrs
                .iter()
                .filter(|attr| attr.path().is_ident("protect"))
            {
                let args = match Args::from_meta(&grants_attr.meta) {
                    Ok(v) => v,
                    Err(e) => {
                        return TokenStream::from(e.write_errors());
                    }
                };

                let generated = ProtectEndpoint::new(args, FnType::Method(method.clone()))
                    .into_token_stream()
                    .into();

                let mut gen_method = parse_macro_input!(generated as syn::ImplItemFn);

                gen_method.attrs.retain(|attr| attr != grants_attr);

                methods.push((idx, gen_method));
            }
        }
    }

    for (idx, method) in methods {
        let _ = std::mem::replace(&mut item_impl.items[idx], syn::ImplItem::Fn(method));
    }

    let res = quote::quote! {
        #item_impl
    };

    res.into()
}
