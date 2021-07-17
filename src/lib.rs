extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::ParseStream, parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(ToUrl)]
pub fn to_url(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs with named fields can be annotated with ToUrl"),
    };

    let fields = fields.iter().map(|field| {
        let field = field.ident.as_ref().unwrap();
        quote! { + &format!("{}={:?}&", stringify!(#field), self.#field) }
    });

    let modified = quote! {
        impl<'a> #name<'a> {
            pub fn to_url(&self, url: String) -> String {

                let url = url #(#fields)*;

                println!("URL: {}", url);

                url
            }
        }
    };
    TokenStream::from(modified)
}
