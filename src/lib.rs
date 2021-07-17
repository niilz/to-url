extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{self, parse::ParseStream, parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(ToUrl)]
pub fn to_url(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let ast = format!("{:?}", input);
    let ident_as_str = format!("{:?}", input.ident);
    let ident = input.ident;

    let fields = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs with named fields can be annotated with ToUrl"),
    };

    let fields_as_str = format!("{:?}", fields);

    let template: String = fields
        .clone()
        .iter()
        .map(|f| f.ident.clone().unwrap().to_string() + "={}&")
        .collect();

    let pups = fields.iter().next().unwrap().clone().ident.unwrap();

    let modified = quote! {
        impl #ident {
            fn to_url(&self, url: &str) {
                println!("my ast: {:?}", #ast);
                println!("my ident: {:?}", #ident_as_str);
                println!("Those are my fields: {:?}", #fields_as_str);

                println!("{}{}{}", url, #template, self.#pups);

            }
        }
    };
    TokenStream::from(modified)
}
