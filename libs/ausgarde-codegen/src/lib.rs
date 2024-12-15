use darling::{ast::NestedMeta, Error, FromMeta};
use proc_macro::TokenStream;
use quote::quote;
use syn::ItemFn;

#[derive(Debug, FromMeta)]
struct AuthorizationProtected {
    #[darling(default)]
    #[darling(multiple)]
    permissions: Vec<String>,
}

/// Protects an `actix-web` route with permissions, from a Bearer token.
///
/// It is important to add `JwtValidator` to your app_data, in order to use this attribute.
/// And name the parameter of the function, that you want to protect, `token`.
///
/// It will return a `Unautharized` response to the client, if the token is not present or if it is invalid.
///
/// # Example
/// ```no_run
/// use actix_web::{web, HttpResponse, get};
///
/// #[get("/")]
/// #[protected(permissions = "permission", permissions = "permission2")]
/// async fn index(token: AccessToken) -> HttpResponse {
///   HttpResponse::Ok().finish()
/// }
#[proc_macro_attribute]
pub fn protected(args: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(Error::from(e).write_errors()),
    };

    let var_name = match AuthorizationProtected::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => return TokenStream::from(e.write_errors()),
    };

    let input = syn::parse_macro_input!(item as ItemFn);

    let fn_vis = &input.vis;
    let fn_attr = &input.attrs;
    let fn_block = &input.block;
    let fn_sig = &input.sig;

    let perms = var_name.permissions;

    let expanded = quote! {
        #(#fn_attr)*
        #fn_vis #fn_sig {
            if token.has_permissions(vec![#(#perms),*]) {
                #fn_block
            } else {
                return Ok(actix_web::HttpResponse::Forbidden().finish());
            }
        }
    };

    TokenStream::from(expanded)
}
