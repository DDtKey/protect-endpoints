use proc_macro2::{Span, TokenStream as TokenStream2, Ident};
use syn::{AttributeArgs, ItemFn, NestedMeta, ReturnType};
use quote::{quote, ToTokens};

pub(crate) struct HasPermissions {
    check_fn: Ident,
    func: ItemFn,
    args: Args
}

impl HasPermissions {
    pub fn new(
        check_fn: &str,
        args: AttributeArgs,
        func: ItemFn
    ) -> syn::Result<Self> {
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
            ReturnType::Type(ref _arrow, ref ty) => {
                ty.to_token_stream()
            }
            ReturnType::Default => { quote! {()}}
        };

        let check_fn = &self.check_fn;

        let args = {
            let permissions = &self.args.permissions;

            quote! {
                #(#permissions,)*
            }
        };

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                _auth_details_: actix_web_grants::permissions::AuthDetails,
                #fn_args
            ) -> actix_web::Either<#fn_output, actix_web::HttpResponse> {
                use actix_web_grants::permissions::{PermissionsCheck, RolesCheck};
                if _auth_details_.#check_fn(vec![#args]) {
                    let f = || async move #func_block;
                    actix_web::Either::A(f().await)
                } else {
                    actix_web::Either::B(actix_web::HttpResponse::Forbidden().finish())
                }
            }
        };

        output.extend(stream);
    }
}

struct Args {
    permissions: Vec<syn::LitStr>,
}

impl Args {
    fn new(args: AttributeArgs) -> syn::Result<Self> {
        let mut permissions = Vec::new();

        for arg in args {
            match arg {
                NestedMeta::Lit(syn::Lit::Str(lit)) => {
                   permissions.push(lit);
                },
                _ => {
                    return Err(syn::Error::new_spanned(arg, "Unknown attribute."));
                }
            }
        }

        Ok(Args {
            permissions
        })
    }
}
