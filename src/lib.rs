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
            let start = std::time::Instant::now();
            let result = { #block };
            println!("Execution de {}: {:?}", stringify!(#name), start.elapsed());
            result
        }
    })
}
