use crate::expand::ProtectEndpoint;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{parse_quote, ReturnType};

impl ProtectEndpoint {
    pub(super) fn to_tokens_salvo(&self, output: &mut TokenStream2) {
        let func_vis = &self.func.vis();
        let func_block = &self.func.block();

        let fn_sig = &self.func.sig();
        let fn_attrs = &self.func.attrs();
        let fn_name = &fn_sig.ident;
        let fn_generics = &fn_sig.generics;
        let fn_async = &fn_sig.asyncness.expect("only async handlers are supported");
        let fn_output = match &fn_sig.output {
            ReturnType::Type(ref _arrow, ref ty) => ty.to_token_stream(),
            ReturnType::Default => {
                quote! {()}
            }
        };

        let ty = self
            .args
            .ty
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or(quote! {String});

        let mut fn_args = fn_sig.inputs.clone();
        let auth_details = format!("_auth_details_{}", fn_args.len());
        let auth_details: Ident = Ident::new(&auth_details, Span::call_site());

        fn_args.push(parse_quote!(#auth_details: protect_salvo::authorities::AuthDetails<#ty>));

        let condition = self
            .args
            .cond
            .to_tokens(&auth_details, self.args.ty.is_some());
        let condition = quote!(if #condition);

        let err_resp = if let Some(expr) = &self.args.error_fn {
            quote!(#expr())
        } else {
            quote!(salvo::prelude::StatusCode::FORBIDDEN)
        };

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                #fn_args
            ) -> Result<#fn_output, impl salvo::Writer + Send + std::fmt::Debug + 'static> {
                use protect_salvo::authorities::AuthoritiesCheck;
                #condition {
                    let f = || async move #func_block;
                    Ok(f().await)
                } else {
                    Err(#err_resp)
                }
            }
        };

        output.extend(stream);
    }
}
