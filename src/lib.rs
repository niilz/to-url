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

    let template: String = fields
        .clone()
        .iter()
        .map(|f| f.ident.clone().unwrap().to_string() + "={}&")
        .collect();

    let pups = fields.iter().next().unwrap().clone().ident.unwrap();

    let modified = quote! {
        impl<'a> #name<'a> {
            pub fn to_url(&self, url: String) -> String {

                println!("{}{}{}", url, #template, self.#pups);

                "dummy".to_string()

            }
        }
    };
    TokenStream::from(modified)
}
