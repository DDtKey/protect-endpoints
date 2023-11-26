use darling::ast::NestedMeta;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use std::ops::Deref;
use syn::{ItemFn, Meta, ReturnType};

const AUTH_DETAILS: &str = "_auth_details_";

#[derive(Debug)]
enum Condition {
    Any(Conditions),
    All(Conditions),
    Value(syn::LitStr),
}

#[derive(Debug)]
struct Conditions(Vec<Condition>);

#[derive(Debug)]
pub(crate) struct Args {
    cond: Condition,
    secure: Option<syn::Expr>,
    ty: Option<syn::Expr>,
}

pub(crate) struct ProtectEndpoint {
    // check_fn: Ident,
    func: ItemFn,
    args: Args,
}

impl ProtectEndpoint {
    pub fn new(args: Args, func: ItemFn) -> syn::Result<Self> {
        Ok(Self { func, args })
    }
}

impl ToTokens for ProtectEndpoint {
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

        let condition = self.args.cond.to_tokens(self.args.ty.is_some());

        let ty = self
            .args
            .ty
            .as_ref()
            .map(syn::Expr::to_token_stream)
            .unwrap_or(quote! {String});

        let condition = if let Some(expr) = &self.args.secure {
            quote!(if #condition && #expr)
        } else {
            quote!(if #condition)
        };
        let auth_details: Ident = Ident::new(AUTH_DETAILS, Span::call_site());

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                #auth_details: rocket_grants::authorities::AuthDetails<#ty>,
                #fn_args
            ) -> Result<#fn_output, rocket::http::Status> {
                use rocket_grants::authorities::AuthoritiesCheck;
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

impl Condition {
    fn to_tokens(&self, is_typed: bool) -> TokenStream2 {
        let auth_details: Ident = Ident::new(AUTH_DETAILS, Span::call_site());

        match self {
            Condition::Any(nested) if nested.iter().all(Condition::is_value) => {
                let vals = nested.iter().map(|c| match c {
                    Condition::Value(val) => val,
                    _ => unreachable!(),
                });

                if is_typed {
                    let vals: Vec<syn::Expr> =
                        vals.map(syn::LitStr::parse).map(Result::unwrap).collect();

                    quote! { #auth_details.has_any_authority(&[#(&#vals,)*]) }
                } else {
                    quote! { #auth_details.has_any_authority(&[#(#vals,)*]) }
                }
            }
            Condition::All(nested) if nested.iter().all(Condition::is_value) => {
                let vals = nested.iter().map(|c| match c {
                    Condition::Value(val) => val,
                    _ => unreachable!(),
                });

                if is_typed {
                    let vals: Vec<syn::Expr> =
                        vals.map(syn::LitStr::parse).map(Result::unwrap).collect();

                    quote! { #auth_details.has_authorities(&[#(&#vals,)*]) }
                } else {
                    quote! { #auth_details.has_authorities(&[#(#vals,)*]) }
                }
            }
            Condition::Any(nested) => {
                let exprs: Vec<_> = nested.iter().map(|c| c.to_tokens(is_typed)).collect();

                quote! { #(#exprs)||* }
            }
            Condition::All(nested) => {
                let exprs: Vec<_> = nested.iter().map(|c| c.to_tokens(is_typed)).collect();

                quote! { #(#exprs)&&* }
            }
            Condition::Value(val) => {
                if is_typed {
                    let val: syn::Expr = val.parse().unwrap();
                    quote! { #auth_details.has_authority(&#val) }
                } else {
                    quote! { #auth_details.has_authority(#val) }
                }
            }
        }
    }

    fn is_value(&self) -> bool {
        matches!(self, Condition::Value(_))
    }
}

impl darling::FromMeta for Condition {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        match *items {
            [] => Err(darling::Error::too_few_items(1)),
            [NestedMeta::Meta(ref meta)] => {
                match darling::util::path_to_string(meta.path()).as_ref() {
                    "any" => Ok(Condition::Any(
                        darling::FromMeta::from_meta(meta).map_err(|e| e.at("any"))?,
                    )),
                    "all" => Ok(Condition::All(
                        darling::FromMeta::from_meta(meta).map_err(|e| e.at("all"))?,
                    )),
                    other => Err(
                        darling::Error::unknown_field_with_alts(other, &["any", "all"])
                            .with_span(meta),
                    ),
                }
            }
            [NestedMeta::Lit(ref lit)] => Ok(Condition::Value(darling::FromMeta::from_value(lit)?)),
            _ => Err(darling::Error::too_many_items(1)),
        }
    }

    fn from_string(value: &str) -> darling::Result<Self> {
        Ok(Condition::Value(darling::FromMeta::from_string(value)?))
    }
}

impl darling::FromMeta for Conditions {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut expressions = Vec::new();
        for item in items {
            let expr = match item {
                nested @ NestedMeta::Meta(_) => Condition::from_list(&[nested.clone()])?,
                NestedMeta::Lit(lit) => Condition::Value(darling::FromMeta::from_value(lit)?),
            };
            expressions.push(expr);
        }

        Ok(Conditions(expressions))
    }
}

impl darling::FromMeta for Args {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut conditions = Vec::new();
        let mut secure = None;
        let mut ty = None;

        let mut errors = ::darling::Error::accumulator();

        for item in items {
            match item {
                NestedMeta::Meta(Meta::NameValue(syn::MetaNameValue { path, value, .. })) => {
                    if path.is_ident("secure") {
                        if secure.is_some() {
                            errors.push(darling::Error::duplicate_field("secure"));
                        } else {
                            secure = errors.handle(darling::FromMeta::from_expr(value));
                        }
                    } else if path.is_ident("ty") {
                        if ty.is_some() {
                            errors.push(darling::Error::duplicate_field("ty"));
                        } else {
                            ty = errors.handle(darling::FromMeta::from_expr(value));
                        }
                    } else {
                        errors.push(darling::Error::unknown_field_path(path));
                    }
                }
                // List may mean either `any` or `all` conditions, so we should try to parse it
                NestedMeta::Meta(Meta::List(_)) => {
                    let cond = errors.handle(darling::FromMeta::from_list(&[item.clone()]));
                    if let Some(cond) = cond {
                        conditions.push(cond);
                    }
                }
                NestedMeta::Lit(syn::Lit::Str(lit)) => {
                    conditions.push(Condition::Value(lit.clone()));
                }
                _ => errors.push(darling::Error::custom(
                    "Unknown attribute, available: 'secure', 'ty', `all`, `any` and string literals",
                )),
            }
        }

        if conditions.is_empty() {
            errors.push(darling::Error::custom(
                "At least one condition must be specified",
            ));
        }

        errors.finish()?;

        let cond = if conditions.len() == 1 {
            conditions.pop().unwrap()
        } else {
            Condition::All(Conditions(conditions))
        };

        Ok(Args { cond, secure, ty })
    }
}

impl Deref for Conditions {
    type Target = Vec<Condition>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
