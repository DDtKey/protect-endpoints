use crate::expand::ProtectEndpoint;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::ReturnType;

impl ProtectEndpoint {
    pub(super) fn to_tokens_actix_web(&self, output: &mut TokenStream2) {
        let func_vis = &self.func.vis();
        let func_block = &self.func.block();

        let fn_sig = &self.func.sig();
        let fn_attrs = &self.func.attrs();
        let fn_name = &fn_sig.ident;
        let fn_generics = &fn_sig.generics;
        let fn_args = &fn_sig.inputs;
        let fn_async = &fn_sig
            .asyncness
            .expect("only async functions are supported");
        let fn_output = match &fn_sig.output {
            ReturnType::Type(ref _arrow, ref ty) => ty.to_token_stream(),
            ReturnType::Default => {
                quote! {()}
            }
        };

        let auth_details = format!("_auth_details_{}", fn_args.len());
        let auth_details: Ident = Ident::new(&auth_details, Span::call_site());

        let ty = self
            .args
            .ty
            .as_ref()
            .map(ToTokens::to_token_stream)
            .unwrap_or(quote! {String});

        let condition = self
            .args
            .cond
            .to_tokens(&auth_details, self.args.ty.is_some());
        let condition = quote!(if #condition);

        let err_resp = if let Some(expr) = &self.args.error_fn {
            quote!(#expr())
        } else {
            quote!(actix_web::HttpResponse::Forbidden().finish())
        };

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                #auth_details: actix_web_grants::authorities::AuthDetails<#ty>,
                #fn_args
            ) -> actix_web::Either<#fn_output, actix_web::HttpResponse> {
                use actix_web_grants::authorities::AuthoritiesCheck;
                #condition {
                    let f = || async move #func_block;
                    actix_web::Either::Left(f().await)
                } else {
                    actix_web::Either::Right(#err_resp)
                }
            }
        };

        output.extend(stream);
    }
}
