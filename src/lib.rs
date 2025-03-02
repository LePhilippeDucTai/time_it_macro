use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn time_it(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let fn_body = &input.block;
    let fn_sig = &input.sig;
    let fn_vis = &input.vis;

    let output = quote! {
        #fn_vis #fn_sig {
            let start = std::time::Instant::now();
            let result = (|| #fn_body)();
            let duration = start.elapsed();
            println!("Execution de {}: {:?}", stringify!(#fn_name), duration);
            result
        }
    };

    TokenStream::from(output)
}
