extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, AttributeArgs, ImplItem, ImplItemMethod, ItemFn, ItemImpl, Meta};

use crate::expand::{FnType, ProtectEndpoint};

mod expand;

const HAS_AUTHORITIES: &str = "has_permissions";
const HAS_ANY_AUTHORITY: &str = "has_any_permission";

const HAS_ROLES: &str = "has_roles";
const HAS_ANY_ROLE: &str = "has_any_role";

macro_rules! unwrap_result {
    ($result:expr) => {
        match $result {
            Ok(result) => result,
            Err(err) => return syn::Error::from(err).to_compile_error().into(),
        }
    };
}

/// Macro to сheck that the user has all the specified permissions.
/// Allow to add a conditional restriction based on handlers parameters.
/// Add the `secure` attribute followed by the the boolean expression to validate based on parameters
///
/// Attention: these macros have to be above of (`[poem::handler]`) or `oai`
///
/// Also you can use you own types instead of Strings, just add `type` attribute with path to type
/// # Examples
/// ```
/// use poem::{Response, http::StatusCode};
///
/// // User should be ADMIN with OP_GET_SECRET permission
/// #[poem_grants::has_permissions["ROLE_ADMIN", "OP_GET_SECRET"]]
/// #[poem::handler]
/// async fn macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
/// }
///
/// // User should be ADMIN with OP_GET_SECRET permission and the user.id param should be equal
/// // to the path parameter {user_id}
/// struct User {id: i32}
/// #[poem_grants::has_permissions["ROLE_ADMIN", "OP_GET_SECRET", secure="*user_id == user.id"]]
/// #[poem::handler]
/// async fn macro_secured_params(user_id: web::Path<i32>, user: web::Data<User>) -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info with user_id path equal to user.id")
///}
///
/// // User must have MyPermissionEnum::OP_GET_SECRET (you own enum example)
/// #[poem_grants::has_permissions["OP_GET_SECRET", type = "MyPermissionEnum"]]
/// #[poem::handler]
/// async fn macro_enum_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
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
/// use poem::{Response, http::StatusCode};
///
/// // User should be ADMIN or MANAGER
/// #[poem_grants::has_any_permission["ROLE_ADMIN", "ROLE_MANAGER"]]
/// #[poem::handler]
/// async fn macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
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
/// use poem::{Response, http::StatusCode};
///
/// // User should be ADMIN and MANAGER
/// #[poem_grants::has_roles["ADMIN", "MANAGER"]]
/// #[poem::handler]
/// async fn macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
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
/// use poem::{Response, http::StatusCode};
///
/// // User should be ADMIN or MANAGER
/// #[poem_grants::has_any_role["ADMIN", "MANAGER"]]
/// #[poem::handler]
/// async fn macro_secured() -> Response {
///     Response::builder().status(StatusCode::OK).body("some secured info")
/// }
/// ```
#[proc_macro_attribute]
pub fn has_any_role(args: TokenStream, input: TokenStream) -> TokenStream {
    check_permissions(HAS_ANY_ROLE, args, input)
}

fn check_permissions(check_fn_name: &str, args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let func = parse_macro_input!(input as ItemFn);

    unwrap_result!(ProtectEndpoint::new(check_fn_name, args, FnType::Fn(func)))
        .into_token_stream()
        .into()
}

/// Macro for `poem-openapi` support
/// Add macro `#[poem_grants::open_api]` above of `#[poem_openapi::OpenApi]` and mark all needed methods with necessary security-methods:
/// One of [`has_permissions`, `has_any_permission`, `has_roles`, `has_any_role`]
///
/// # Examples
/// ```
/// use poem_openapi::payload::PlainText;
///
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
#[proc_macro_attribute]
pub fn open_api(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(input as ItemImpl);
    let mut methods = Vec::new();
    for (idx, item) in item_impl.items.iter().enumerate() {
        if let ImplItem::Method(method) = item {
            for grants_attr in method
                .attrs
                .iter()
                .filter(|attr| is_poem_grants_attr(*attr))
            {
                let args = match unwrap_result!(grants_attr.parse_meta()) {
                    Meta::List(list) => list.nested.into_iter().collect::<Vec<syn::NestedMeta>>(),
                    _ => {
                        return syn::Error::new_spanned(
                            grants_attr,
                            "Expected endpoint-attribute to be a list",
                        )
                        .to_compile_error()
                        .into()
                    }
                };

                let generated = unwrap_result!(ProtectEndpoint::new(
                    &grants_attr
                        .path
                        .get_ident()
                        .expect("validated by condition above")
                        .to_string(),
                    args,
                    FnType::Method(method.clone()),
                ))
                .into_token_stream()
                .into();

                let mut gen_method = parse_macro_input!(generated as ImplItemMethod);

                gen_method.attrs.retain(|attr| attr != grants_attr);

                methods.push((idx, gen_method));
            }
        }
    }

    for (idx, method) in methods {
        let _ = std::mem::replace(&mut item_impl.items[idx], ImplItem::Method(method));
    }

    let res = quote::quote! {
        #item_impl
    };

    res.into()
}

fn is_poem_grants_attr(attr: &syn::Attribute) -> bool {
    attr.path.is_ident(HAS_ANY_AUTHORITY)
        || attr.path.is_ident(HAS_AUTHORITIES)
        || attr.path.is_ident(HAS_ANY_ROLE)
        || attr.path.is_ident(HAS_ROLES)
}
