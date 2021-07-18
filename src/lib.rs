extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse::ParseStream, parse_macro_input, punctuated::Punctuated, token::Comma, Data,
    DataStruct, DeriveInput, Field, Fields, Path, PathSegment, Type, TypePath,
};

#[proc_macro_derive(ToUrl)]
pub fn to_url(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as DeriveInput);

    let name = input.ident;

    let fields_punct = match input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => fields.named,
        _ => panic!("Only structs with named fields can be annotated with ToUrl"),
    };

    let fields_str = format!("{:?}", fields_punct);

    let query_parts = query_from_field_and_value(&fields_punct);

    let modified = quote! {
        impl<'a> #name<'a> {
            pub fn to_url(&self, url: String) -> String {

                let url = format!("{}?", url) #(#query_parts)*;

                println!("URL: {}", url);
                println!("fields_str: {}", #fields_str);

                url
            }
        }
    };
    TokenStream::from(modified)
}

fn query_from_field_and_value(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let fields = fields.iter().map(|field| {
        let field_ident = field.ident.as_ref().unwrap();
        if let Some(ref path_seg) = get_vec(&field) {
            join_values(path_seg).unwrap()
        } else {
            quote! { + &format!("{}={:?}&", stringify!(#field), self.#field_ident) }
        }
    });
    fields
}

fn get_vec(field: &Field) -> Option<&PathSegment> {
    let field_ident = field.ident.as_ref().unwrap();
    match &field.ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            // segments is of Type syn::punctuated::Punctuated<PathSegment, _>
            if let Some(path_seg) = segments.first() {
                let ident = &path_seg.ident;
                if ident == "Vec" {
                    return Some(path_seg);
                }
            }
            None
        }
        _ => None,
    }
}

fn join_values(vec: &PathSegment) -> Option<proc_macro2::TokenStream> {
    let ident = &vec.ident;
    Some(quote! {+ &format!("###{:?}###", stringify!(#ident))})
}
