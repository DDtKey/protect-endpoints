use darling::ast::NestedMeta;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::quote;
use std::ops::Deref;
use syn::{Block, ItemFn, Meta};

#[cfg(feature = "actix-web")]
mod actix_web;
#[cfg(feature = "poem")]
mod poem;
#[cfg(feature = "rocket")]
mod rocket;

#[derive(Clone)]
pub(crate) enum FnType {
    Fn(ItemFn),
    #[cfg(feature = "poem")]
    Method(syn::ImplItemFn),
}

#[derive(Debug)]
enum Condition {
    Any(Conditions),
    All(Conditions),
    Expr(syn::Expr),
    Value(syn::LitStr),
}

#[derive(Debug)]
struct Conditions(Vec<Condition>);

#[derive(Debug)]
pub(crate) struct Args {
    cond: Condition,
    ty: Option<syn::Expr>,
    error_fn: Option<Ident>,
}

pub(crate) struct ProtectEndpoint {
    func: FnType,
    args: Args,
}

impl ProtectEndpoint {
    pub fn new(args: Args, func: FnType) -> Self {
        Self { func, args }
    }
}

impl Condition {
    fn to_tokens(&self, auth_details: &Ident, is_typed: bool) -> TokenStream2 {
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
                let exprs: Vec<_> = nested
                    .iter()
                    .map(|c| c.to_tokens(auth_details, is_typed))
                    .collect();

                quote! { #(#exprs)||* }
            }
            Condition::All(nested) => {
                let exprs: Vec<_> = nested
                    .iter()
                    .map(|c| c.to_tokens(auth_details, is_typed))
                    .collect();

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
            Condition::Expr(expr) => {
                quote! { #expr }
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
                    "expr" => Ok(Condition::Expr(
                        darling::FromMeta::from_meta(meta).map_err(|e| e.at("expr"))?,
                    )),
                    other => Err(darling::Error::unknown_field_with_alts(
                        other,
                        &["any", "all", "expr"],
                    )
                    .with_span(meta)),
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
        let mut ty = None;
        let mut error_fn = None;

        let mut errors = ::darling::Error::accumulator();

        for item in items {
            match item {
                NestedMeta::Meta(Meta::NameValue(syn::MetaNameValue { path, value, .. })) => {
                    if path.is_ident("ty") {
                        if ty.is_some() {
                            errors.push(darling::Error::duplicate_field("ty"));
                        } else {
                            ty = errors.handle(darling::FromMeta::from_expr(value));
                        }
                    } else if path.is_ident("error") {
                        if ty.is_some() {
                            errors.push(darling::Error::duplicate_field("error"));
                        } else {
                            error_fn = errors.handle(darling::FromMeta::from_expr(value));
                        }
                    } else if path.is_ident("expr") {
                        let cond = errors
                            .handle(darling::FromMeta::from_expr(value))
                            .map(Condition::Expr);
                        if let Some(cond) = cond {
                            conditions.push(cond);
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
                NestedMeta::Lit(lit) => {
                    let cond = errors
                        .handle(darling::FromMeta::from_value(lit))
                        .map(Condition::Value);
                    if let Some(cond) = cond {
                        conditions.push(cond);
                    }
                }
                _ => errors.push(darling::Error::custom(
                    "Unknown attribute, available: 'ty', `all`, `any`, `expr` and string literals",
                )),
            }
        }

        if conditions.is_empty() {
            errors.push(darling::Error::custom("At least one condition must be specified"));
        }

        errors.finish()?;

        let cond = if conditions.len() == 1 {
            conditions.pop().unwrap()
        } else {
            Condition::All(Conditions(conditions))
        };

        Ok(Args { cond, ty, error_fn })
    }
}

impl Deref for Conditions {
    type Target = Vec<Condition>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FnType {
    fn sig(&self) -> &syn::Signature {
        match self {
            FnType::Fn(fun) => &fun.sig,
            #[cfg(feature = "poem")]
            FnType::Method(fun) => &fun.sig,
        }
    }

    fn vis(&self) -> &syn::Visibility {
        match self {
            FnType::Fn(fun) => &fun.vis,
            #[cfg(feature = "poem")]
            FnType::Method(fun) => &fun.vis,
        }
    }

    fn block(&self) -> &Block {
        match self {
            FnType::Fn(fun) => &fun.block,
            #[cfg(feature = "poem")]
            FnType::Method(fun) => &fun.block,
        }
    }

    fn attrs(&self) -> &Vec<syn::Attribute> {
        match self {
            FnType::Fn(fun) => &fun.attrs,
            #[cfg(feature = "poem")]
            FnType::Method(fun) => &fun.attrs,
        }
    }
}
