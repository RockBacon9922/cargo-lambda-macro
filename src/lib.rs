extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, parse_macro_input};

#[proc_macro_attribute]
pub fn lambda_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let expanded = quote! {
        #input_fn

        #[tokio::main]
        async fn main() -> Result<(), lambda_runtime::Error> {
            let func = lambda_runtime::service_fn(#fn_name);
            lambda_runtime::run(func).await?;
            Ok(())
        }
    };

    TokenStream::from(expanded)
}
