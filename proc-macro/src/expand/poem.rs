use crate::expand::ProtectEndpoint;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{parse_quote, PathArguments, ReturnType, Type};

impl ToTokens for ProtectEndpoint {
    fn to_tokens(&self, output: &mut TokenStream2) {
        let func_vis = &self.func.vis();
        let func_block = &self.func.block();
        let fn_sig = &self.func.sig();
        let fn_attrs = &self.func.attrs();

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
        let auth_details = format!("_auth_details_{}", fn_args.len());
        let auth_details: Ident = Ident::new(&auth_details, Span::call_site());

        fn_args.push(parse_quote!(#auth_details: poem_grants::authorities::AuthDetails<#ty>));

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
                    .filter(|last_seg| last_seg.ident == "Result")
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

        let condition = self
            .args
            .cond
            .to_tokens(&auth_details, self.args.ty.is_some());
        let condition = quote!(if #condition);

        let err_resp = if let Some(expr) = &self.args.error_fn {
            quote!(#expr())
        } else {
            quote!(poem::Error::from(poem_grants::error::AccessError::ForbiddenRequest))
        };

        let stream = quote! {
            #(#fn_attrs)*
            #func_vis #fn_async fn #fn_name #fn_generics(
                #fn_args
            ) -> poem::Result<#fn_output> {
                use poem::error::IntoResult;
                use poem_grants::authorities::AuthoritiesCheck;
                #condition {
                    let f = || async move #func_block;
                    let val: #original_out = f().await;
                    val.into_result()
                } else {
                    Err(#err_resp)
                }
            }
        };

        output.extend(stream);
    }
}
