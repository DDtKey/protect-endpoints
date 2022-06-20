use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemFn, NestedMeta, ReturnType};

pub(crate) struct HasPermissions {
    check_fn: Ident,
    func: ItemFn,
    args: Args,
}

impl HasPermissions {
    pub fn new(check_fn: &str, args: AttributeArgs, func: ItemFn) -> syn::Result<Self> {
        let check_fn: Ident = syn::parse_str(check_fn)?;

        let args = Args::new(args)?;
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

        let args = if self.args.type_.is_some() {
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

        let type_ = self
            .args
            .type_
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
                _auth_details_: rocket_grants::permissions::AuthDetails<#type_>,
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

struct Args {
    permissions: Vec<syn::LitStr>,
    secure: Option<syn::Expr>,
    type_: Option<syn::Expr>,
}

impl Args {
    fn new(args: AttributeArgs) -> syn::Result<Self> {
        let mut permissions = Vec::with_capacity(args.len());
        let mut secure = None;
        let mut type_ = None;
        for arg in args {
            match arg {
                NestedMeta::Lit(syn::Lit::Str(lit)) => {
                    permissions.push(lit);
                }
                NestedMeta::Meta(syn::Meta::NameValue(syn::MetaNameValue {
                    path,
                    lit: syn::Lit::Str(lit_str),
                    ..
                })) => {
                    if path.is_ident("secure") {
                        let expr = lit_str.parse().unwrap();
                        secure = Some(expr);
                    } else if path.is_ident("type") {
                        let expr = lit_str.parse().unwrap();
                        type_ = Some(expr);
                    } else {
                        return Err(syn::Error::new_spanned(
                            path,
                            "Unknown identifier. Available: 'secure' and 'type'",
                        ));
                    }
                }
                _ => {
                    return Err(syn::Error::new_spanned(arg, "Unknown attribute."));
                }
            }
        }

        Ok(Args {
            permissions,
            secure,
            type_,
        })
    }
}
