use proc_macro2::{Span, TokenStream as TokenStream2, Ident};
use syn::{AttributeArgs, ItemFn, NestedMeta};
use quote::{quote, ToTokens};

pub(crate) struct HasAuthorities {
    check_fn: Ident,
    func: ItemFn,
    args: Args
}

impl HasAuthorities {
    pub fn new(
        check_fn: &str,
        args: AttributeArgs,
        func: ItemFn
    ) -> syn::Result<Self> {
        let check_fn: Ident = syn::parse_str(check_fn)?;

        let args = Args::new(args)?;
        if args.authorities.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "The #[has_authorities(..)] macro requires at least one `authority` argument",
            ));
        }

        Ok(Self {
            check_fn,
            func,
            args,
        })
    }
}

impl ToTokens for HasAuthorities {
    fn to_tokens(&self, output: &mut TokenStream2) {
        let func_vis = &self.func.vis;
        let func_block = &self.func.block;

        let fn_sig = &self.func.sig;
        let fn_attrs = &self.func.attrs;
        let fn_name = &fn_sig.ident;
        let fn_generics = &fn_sig.generics;
        let fn_args = &fn_sig.inputs;
        let fn_output = &fn_sig.output;
        let fn_async = &fn_sig.asyncness.unwrap();

        let check_fn = &self.check_fn;

        let args = {
            let authorities = &self.args.authorities;

            quote! {
                #(#authorities,)*
            }
        };

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                _auth_details_: actix_web_grants::authorities::AuthDetails,
                #fn_args
            ) #fn_output {
                use actix_web_grants::authorities::{AuthoritiesCheck, RolesCheck};
                if _auth_details_.#check_fn(vec![#args]) {
                    #func_block
                } else {
                    actix_web::HttpResponse::Forbidden().finish()
                }
            }
        };

        output.extend(stream);
    }
}

struct Args {
    authorities: Vec<syn::LitStr>,
}

impl Args {
    fn new(args: AttributeArgs) -> syn::Result<Self> {
        let mut authorities = Vec::new();

        for arg in args {
            match arg {
                NestedMeta::Lit(syn::Lit::Str(lit)) => {
                   authorities.push(lit);
                },
                _ => {
                    return Err(syn::Error::new_spanned(arg, "Unknown attribute."));
                }
            }
        }

        Ok(Args {
            authorities
        })
    }
}
