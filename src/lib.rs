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

        fn generate_default_lambda_event() -> lambda_runtime::LambdaEvent<serde_json::Value> {
            lambda_runtime::LambdaEvent::new(serde_json::Value::default(), lambda_runtime::Context::default())
        }

        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            // Check if running in lambda environment
            let is_lambda = std::env::vars()
                .any(|(key, _)| key.starts_with("AWS_LAMBDA"));

            println!("Running in Lambda environment: {}", is_lambda);


            if is_lambda {
                let func = lambda_runtime::service_fn(#fn_name);
                lambda_runtime::run(func).await?;
            } else {
                let event = generate_default_lambda_event();
                let result = #fn_name(event).await?;
                print!("{:?}", result);
            }

            Ok(())
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn local_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;

    let expanded = quote! {
        #input_fn


        fn generate_default_lambda_event() -> lambda_runtime::LambdaEvent<Value> {
            lambda_runtime::LambdaEvent::new(serde_json::Value::default(), lambda_runtime::Context::default())
        }

        #[tokio::main]
        async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let event = generate_default_lambda_event();
            let result = #fn_name(event).await?;

            print!("{:?}", result);
            Ok(())
        }
    };

    TokenStream::from(expanded)
}
