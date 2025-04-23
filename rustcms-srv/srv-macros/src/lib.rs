extern crate proc_macro;

use ::proc_macro::TokenStream;
use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use ::syn::{
    ItemFn, Meta, Token, parse::Parser, parse_macro_input, parse_quote, punctuated::Punctuated,
};

/// ## Examples
///
/// ```rust,ignore
/// #[handler(state, claims, permission = "public:view")]
/// ```
/// ```rust,ignore
/// #[handler(permission = "storage:view")]
/// ```
/// ```rust,ignore
/// #[handler(result)]
/// ```
/// ```rust,ignore
/// #[handler(permission = "public:edit", result = StatusCode::CREATED)]
/// ```
#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut result_token: TokenStream2 = Default::default();
    let mut check_permission_token: TokenStream2 = Default::default();

    let args_parsed = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse(args)
        .unwrap();

    let input = parse_macro_input!(input as ItemFn);

    let ItemFn {
        attrs,
        vis,
        mut sig,
        block,
    } = input;

    let stmts = &block.stmts;

    args_parsed.iter().for_each(|meta| {
        let path = meta.path();

        if path.is_ident("state") {
            sig.inputs.insert(0, parse_quote! {
                ::axum::extract::State(state): ::axum::extract::State<::std::sync::Arc<AppState>>
            });
        }

        if path.is_ident("claims") {
            sig.inputs.insert(
                0,
                parse_quote! {
                    claims: crate::services::middleware::Claims<'_>
                },
            );
        }

        if path.is_ident("result") {
            if let Ok(result) = meta.require_name_value() {
                let value = &result.value;

                result_token = quote! {
                    Ok(#value)
                }

            } else {
                result_token = quote! {
                    Ok(())
                }
            }
        }

        if path.is_ident("permission") {
            let permission = &meta
                .require_name_value()
                .expect("permission value must be set")
                .value;

            check_permission_token = quote! {
                if let Some(crate::services::middleware::AuthState { roles: Some(__roles), .. }) = &claims.auth {
                    use crate::repositories::cache::CacheRepository;
                    state.cache.check_roles_has_permission(__roles, #permission).await?;
                } else {
                    Err(crate::app::AuthError::AccessForbidden)?
                }
            };
        }
    });

    sig.output = parse_quote! { -> Result<impl ::axum::response::IntoResponse, crate::app::Error> };

    quote! {
        #(#attrs)* #vis #sig {
            #check_permission_token

            #(#stmts)*

            #result_token
        }
    }
    .into()
}
