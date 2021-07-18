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
This method, if called with a `String` first adds a `?` (question mark), the start of a query.
It then iterates over all fields and joins them in the form `field=value` and adds them to the base_url.  
If the `struct` has more than one field, the pairs are concatenated with an `&` (ampersand).  
`Vec`s are treated specially (only one dimensional `Vec`s are supported): Their values are all joined with a url-encoded space character which is the `%20`.


## Example

```rust
// This example must be in a different crate than *to-url* (because proc_macros must be defined in their own crates)


// Annotate a struct with named fields
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

First of all this macro should probably not being used for anything but experimenting or learning.
But the example here came from the idea to contruct an [openid-connect request url](https://developers.google.com/identity/protocols/oauth2/openid-connect).
And since I already had a `struct` in my dummy-project, with all the fields named like the query-parameters, I thougth it would be nice to use the field names directly. And proc_macros are the only option to do so.
Other solutions (most likely more suited here) are:
- have a simple manually coded `to_url` method where the field names are just typed out again
- use a `HashMap` that contains the necessary request-parameters. That might actually get rid of the struct all together if it is just a data-holder and does not provide much more functionality.

For this specific use case though, the macro works fine, it expresses the intend and the benefit is a typed `struct` where one can add or remove fields and the will be automatically embedded/excluded in the final query.
