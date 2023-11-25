use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{ExprLit, ItemFn, ReturnType};

pub(crate) struct HasPermissions {
    check_fn: Ident,
    func: ItemFn,
    args: Args,
}

impl HasPermissions {
    pub fn new(check_fn: &str, args: Args, func: ItemFn) -> syn::Result<Self> {
        let check_fn: Ident = syn::parse_str(check_fn)?;

        if args.permissions.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "The #[has_permissions(..)] macro requires at least one `permission` argument",
            ));
        }

        Ok(Self {
            check_fn,
            func,
            args,
        })
    }
}

impl ToTokens for HasPermissions {
    fn to_tokens(&self, output: &mut TokenStream2) {
        let func_vis = &self.func.vis;
        let func_block = &self.func.block;

        let fn_sig = &self.func.sig;
        let fn_attrs = &self.func.attrs;
        let fn_name = &fn_sig.ident;
        let fn_generics = &fn_sig.generics;
        let fn_args = &fn_sig.inputs;
        let fn_async = &fn_sig.asyncness.unwrap();
        let fn_output = match &fn_sig.output {
            ReturnType::Type(ref _arrow, ref ty) => ty.to_token_stream(),
            ReturnType::Default => {
                quote! {()}
            }
        };

        let check_fn = &self.check_fn;

        let args = if self.args.ty.is_some() {
            let permissions: Vec<syn::Expr> = self
                .args
                .permissions
                .iter()
                .map(|perm| perm.parse().unwrap())
                .collect();

            quote! {
                #(&#permissions,)*
            }
        } else {
            let permissions = &self.args.permissions;

            quote! {
                #(#permissions,)*
            }
        };

        let ty = self
            .args
            .ty
            .as_ref()
            .map(|t| t.to_token_stream())
            .unwrap_or(quote! {String});

        let condition = if let Some(expr) = &self.args.secure {
            quote!(if _auth_details_.#check_fn(&[#args]) && #expr)
        } else {
            quote!(if _auth_details_.#check_fn(&[#args]))
        };

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                _auth_details_: rocket_grants::permissions::AuthDetails<#ty>,
                #fn_args
            ) -> Result<#fn_output, rocket::http::Status> {
                use rocket_grants::permissions::{PermissionsCheck, RolesCheck};
                #condition {
                    let f = || async move #func_block;
                    Ok(f().await)
                } else {
                    Err(rocket::http::Status::Forbidden)
                }
            }
        };

        output.extend(stream);
    }
}

#[derive(Default)]
pub(crate) struct Args {
    permissions: Vec<syn::LitStr>,
    secure: Option<syn::Expr>,
    ty: Option<syn::Expr>,
}

impl darling::FromMeta for Args {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut permissions = Vec::new();
        let mut secure = None;
        let mut ty = None;

        for item in items {
            match item {
                NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    value:
                        syn::Expr::Lit(ExprLit {
                            lit: syn::Lit::Str(lit_str),
                            ..
                        }),
                    ..
                })) => {
                    if path.is_ident("secure") {
                        let expr = lit_str.parse().unwrap();
                        secure = Some(expr);
                    } else if path.is_ident("ty") {
                        let expr = lit_str.parse().unwrap();
                        ty = Some(expr);
                    } else {
                        return Err(darling::Error::unknown_field_path(path));
                    }
                }
                NestedMeta::Lit(syn::Lit::Str(lit)) => {
                    permissions.push(lit.clone());
                }
                _ => {
                    return Err(darling::Error::custom(
                        "Unknown attribute, available: 'secure', 'ty' & string literal",
                    ))
                }
            }
        }

        Ok(Args {
            permissions,
            secure,
            ty,
        })
    }
}
