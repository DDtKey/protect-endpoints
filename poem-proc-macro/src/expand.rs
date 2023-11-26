use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::convert::{TryFrom, TryInto};
use syn::{
    parse_quote, AttributeArgs, ImplItemMethod, ItemFn, NestedMeta, PathArguments, ReturnType, Type,
};

pub(crate) struct ProtectEndpoint {
    check_fn: Ident,
    func: FnType,
    args: EndpointArgs,
}

pub(crate) struct EndpointArgs {
    permissions: Vec<syn::LitStr>,
    secure: Option<syn::Expr>,
    ty: Option<syn::Expr>,
}

#[derive(Clone)]
pub(crate) enum FnType {
    Fn(ItemFn),
    Method(ImplItemMethod),
}

impl ProtectEndpoint {
    pub fn new<Args>(check_fn: &str, args: Args, func: FnType) -> syn::Result<Self>
    where
        Args: TryInto<EndpointArgs>,
        syn::Error: From<<Args as TryInto<EndpointArgs>>::Error>,
    {
        let check_fn: Ident = syn::parse_str(check_fn)?;
        let args = args.try_into()?;

        if args.permissions.is_empty() {
            return Err(syn::Error::new(
                Span::call_site(),
                "`poem_grants` macro requires at least one `permission/role` argument",
            ));
        }

        Ok(Self {
            check_fn,
            func,
            args,
        })
    }
}

impl ToTokens for ProtectEndpoint {
    fn to_tokens(&self, output: &mut TokenStream2) {
        let (func_vis, func_block, fn_sig, fn_attrs) = match self.func.clone() {
            FnType::Fn(func) => (func.vis, func.block, func.sig, func.attrs),
            FnType::Method(func) => (func.vis, Box::new(func.block), func.sig, func.attrs),
        };

        let fn_name = &fn_sig.ident;
        let fn_generics = &fn_sig.generics;
        let fn_async = &fn_sig.asyncness.unwrap();

        let ty = self
            .args
            .ty
            .as_ref()
            .map(|t| t.to_token_stream())
            .unwrap_or(quote! {String});

        let mut fn_args = fn_sig.inputs.clone();
        fn_args.push(parse_quote!(_auth_details_: poem_grants::permissions::AuthDetails<#ty>));

        let (original_out, fn_output) = match &fn_sig.output {
            ReturnType::Type(ref _arrow, ref ty) => {
                let fn_out = Some(ty.as_ref())
                    .and_then(|ty| {
                        if let Type::Path(ty_path) = ty {
                            ty_path.path.segments.last()
                        } else {
                            None
                        }
                    })
                    .filter(|last_seg| last_seg.ident.to_string() == "Result")
                    .and_then(|last_seg| {
                        if let PathArguments::AngleBracketed(angle_args) =
                            last_seg.clone().arguments
                        {
                            if let Some(syn::GenericArgument::Type(res_out_ty)) =
                                angle_args.args.first()
                            {
                                Some(res_out_ty.to_token_stream())
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .unwrap_or_else(|| ty.to_token_stream());

                (ty.to_token_stream(), fn_out)
            }
            ReturnType::Default => (quote! {()}, quote! {()}),
        };

        let check_fn = &self.check_fn;

        let check_args = if self.args.ty.is_some() {
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

        let condition = if let Some(expr) = &self.args.secure {
            quote!(if _auth_details_.#check_fn(&[#check_args]) && #expr)
        } else {
            quote!(if _auth_details_.#check_fn(&[#check_args]))
        };

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                #fn_args
            ) -> poem::Result<#fn_output> {
                use poem::error::IntoResult;
                use poem_grants::permissions::{PermissionsCheck, RolesCheck};
                #condition {
                    let f = || async move #func_block;
                    let val: #original_out = f().await;
                    val.into_result()
                } else {
                    Err(poem::Error::from(poem_grants::error::AccessError::ForbiddenRequest))
                }
            }
        };

        output.extend(stream);
    }
}

impl TryFrom<AttributeArgs> for EndpointArgs {
    type Error = syn::Error;

    fn try_from(value: AttributeArgs) -> Result<Self, Self::Error> {
        EndpointArgs::new(value)
    }
}

impl EndpointArgs {
    fn new(args: AttributeArgs) -> syn::Result<Self> {
        let mut permissions = Vec::with_capacity(args.len());
        let mut secure = None;
        let mut ty = None;
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
                        ty = Some(expr);
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

        Ok(EndpointArgs {
            permissions,
            secure,
            ty,
        })
    }
}
