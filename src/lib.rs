extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::ParseStream, parse_macro_input, ItemStruct};

#[proc_macro_derive(ToUrl)]
pub fn to_url(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ItemStruct);
    let ident = input.ident;
    let modified = quote! {
        impl #ident {
            fn demo(&self) {
                println!("I have made macro");
            }
        }
    };
    TokenStream::from(modified)
}
