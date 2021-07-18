extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    self, parse_macro_input, punctuated::Punctuated, token::Comma, Data, DataStruct, DeriveInput,
    Field, Fields, Ident, Path, Type, TypePath,
};

const URL_SPACE: &'static str = "%20";

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

    let query_parts = query_from_field_and_value(&fields_punct);

    let modified = quote! {
        impl<'a> #name<'a> {
            pub fn to_url(&self, base_url: String) -> String {

                let url = format!("{}?", base_url) #(#query_parts)*;

                url
            }
        }
    };
    TokenStream::from(modified)
}

fn query_from_field_and_value(
    fields: &Punctuated<Field, Comma>,
) -> impl Iterator<Item = proc_macro2::TokenStream> + '_ {
    let fields = fields.iter().enumerate().map(move |(i, field)| {
        let field_ident = field.ident.as_ref().unwrap();
        let delim = if i < fields.len() - 1 { "&" } else { "" };
        if is_vec(&field) {
            join_values(field_ident)
        } else {
            quote! { + &format!("{}={}{}", stringify!(#field_ident), self.#field_ident, #delim) }
        }
    });
    fields
}

fn is_vec(field: &Field) -> bool {
    match &field.ty {
        Type::Path(TypePath {
            path: Path { segments, .. },
            ..
        }) => {
            // segments is of Type syn::punctuated::Punctuated<PathSegment, _>
            if let Some(path_seg) = segments.first() {
                let ident = &path_seg.ident;
                return ident == "Vec";
            }
            false
        }
        _ => false,
    }
}

fn join_values(field_ident: &Ident) -> proc_macro2::TokenStream {
    let len = quote! { self.#field_ident.len() };
    let vec_values = quote! {
        //let len = self.#field_ident.len();
        self.#field_ident.iter().enumerate().fold(String::new(), |mut vals, (i, v)| {
            vals.push_str(v);
            if (i < #len - 1) {
                vals.push_str(#URL_SPACE);
            }
            if (i == #len - 1) {
                vals.push('&');
            }
            vals
        })
    };
    quote! {+ &format!("{}={}", stringify!(#field_ident), #vec_values)}
}
