extern crate proc_macro;

use ::proc_macro::TokenStream;
use ::proc_macro2::TokenStream as TokenStream2;
use ::quote::quote;
use ::syn::{
    parse::Parser, parse_macro_input, parse_quote, punctuated::Punctuated, ItemFn, Meta, Token,
};

/// Defines a handler function
/// 
/// ## Examples
/// 
/// ```rust,ignore
/// #[handler(state, claims, permission = "public:view")]
/// ```
/// ```rust,ignore
/// #[handler(permission = "storage:edit")]
/// ```
/// 
/// - `state` - injects a reference to [`Arc<AppState>`]
/// - `claims` - injects a reference to [`Claims<'_>`]
/// - `permission = "public:view"` - implements check permission routine 
#[proc_macro_attribute]
pub fn handler(args: TokenStream, input: TokenStream) -> TokenStream {
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

    let mut check_permission_token: TokenStream2 = Default::default();

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

        if path.is_ident("permission") {
            let permission = meta
                .require_name_value()
                .expect("permission value must be set")
                .value
                .clone();

            check_permission_token = quote! {
                if let Some(crate::models::AuthState { roles: Some(__roles), .. }) = claims.auth.clone() {
                    use crate::repositories::cache::CacheRepository;
                    state.db.check_roles_has_permission(&__roles, #permission).await?;
                } else {
                    Err(crate::app::AuthError::AccessForbidden)?
                }
            };
        }
    });

    sig.output = parse_quote! { -> Result<impl ::axum::response::IntoResponse> };

    quote! {
        #(#attrs)* #vis #sig {
            #check_permission_token

            #(#stmts)*
        }
    }
        .into()
}
