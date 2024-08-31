use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// Macro for adding logging functionality
#[proc_macro_attribute]
pub fn logging(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_args = &input.sig.inputs;
    let fn_return = &input.sig.output;
    let fn_body = &input.block;

    let gen = quote! {
        fn #fn_name(#fn_args) #fn_return {
            println!("Logging: Calling function: {}", stringify!(#fn_name));
            let result = (|| #fn_body)();
            println!("Logging: Function {} completed", stringify!(#fn_name));

            result
        }
    };

    gen.into()
}

// Macro for adding caching functionality
#[proc_macro_attribute]
pub fn cache(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_args = &input.sig.inputs;
    let fn_return = &input.sig.output;

    let gen = quote! {
        use std::collections::HashMap;
        use std::sync::Mutex;

        lazy_static::lazy_static! {
            static ref CACHE: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
        }

        fn #fn_name(#fn_args) -> #fn_return {
            let input = format!("{:?}", #fn_args);

            if let Some(cached_result) = CACHE.lock().unwrap().get(input) {
                println!("Cache hit for input: {}", input);
                return cached_result.clone();
            }

            println!("Cache miss for input: {}", input);
            
            let result = (|| #fn_body)();  // call the original function
            
            CACHE.lock().unwrap().insert(input.to_string(), result.clone());

            result
        }
    };

    gen.into()
}
