# ToUrl - A Rust proc_macro_derive example

This library has been developed to try out how procedural macros in Rust, specifically the derive-macro, can be realized.

## Used Libraries:

- [syn 1.0 (full, extra-traits)](https://docs.rs/syn/1.0.73/syn/index.html)
- [quote 1.0](https://docs.rs/quote/1.0.9/quote/index.html)
- [proc_macro2 1.0](https://docs.rs/proc-macro2/1.0.27/proc_macro2/index.html)


## What it does

Note: This is not a macro for production!

If the derive attribute `ToUrl` is added to a struct (only structs are supported) then a  
`to_url(&self, base_url: String) -> String` method gets implemented for that struct.  
When called with a URL-`String` (the `url_base`), this method will first add a `?` (start of a url-query-part), to the given URL.
It then iterates over all fields and adds them in the form `field=value`, to the current url-`String`.  
If the `struct` has more than one field, the pairs are concatenated with an `&` (ampersand).  
`Vec`s are treated slightly special (only one dimensional `Vec`s are supported):  
Their values are all joined with a url-encoded space character (`%20`).


## Example

```rust
// This example must be in a different crate than *to-url* (because proc_macros must be defined in their own crates)


// Annotate a struct (with named fields) with the ToUrl-derive (macro)
#[derive(ToUrl)]
pub struct Request<'a> {
    response_type: &'a str,
    client_id: &'a str,
    scope: Vec<&'a str>,
    redirect_uri: &'a str,
    state: String,
    nonce: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_url_from_fields_and_values() {

		    // Create an instance of the annotated sruct
        let dummy_req = Request {
            response_type: "code",
            client_id: "1234andSomeText",
            scope: vec!["openid", "email", "profile"],
            redirect_uri: "http://dummy-redirect.com",
            state: "security_token0815".to_string(),
            nonce: "80085-3531".to_string(),
        };

        // Call `to_url` on the insance, passing in a base-url
        let url = dummy_req.to_url("https://dummy-base-url".to_string());

        // All fields are collected into one url with query string
        assert_eq!(
						url,
            "https://dummy-base-url?\
                response_type=code&\
                client_id=1234andSomeText&\
                scope=openid%20email%20profile&\
                redirect_uri=http://dummy-redirect.com&\
                state=security_token0815&\
                nonce=80085-3531"
                .to_string()
        );
    }
}
```

## What is this macro good for?

First of all this macro should probably not be used for anything else but experimenting or learning.
But the example here came from the idea to construct an [openid-connect request url](https://developers.google.com/identity/protocols/oauth2/openid-connect).
And since I already had a `struct` in my dummy-project, with all the fields named like the query-parameters, I thougth it would be nice to use the field names directly. And proc_macros are the only option to do so.
Other solutions (most likely even more suited here) are:
- have a simple manually coded `to_url` method on the struct, where the field names are just repeated again by hand.
- use a `HashMap` that contains the necessary request-parameters. That might actually get rid of the struct all together if it is just a data-holder and does not provide much more functionality.

For this specific use case though, the macro works fine.
It expresses the intend with the benefit of having a typed `struct`, where one can add or remove fields and they will be automatically embedded/excluded in the final url-query-part.
