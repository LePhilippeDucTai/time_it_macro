use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn time_it(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let func = parse_macro_input!(item as ItemFn);
    let name = &func.sig.ident;
    let block = &func.block;
    let sig = &func.sig;
    let vis = &func.vis;
    TokenStream::from(quote! {
        #vis #sig {
            use tracing::{info};
            use tracing_subscriber;
            tracing_subscriber::fmt::init();
            let start = std::time::Instant::now();
            let result = { #block };
            info!("Executing function {}: {:?}", stringify!(#name), start.elapsed().round(2));
            result
        }
    })
}
