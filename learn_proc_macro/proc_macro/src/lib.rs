extern crate proc_macro;

// use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Item};

/// This procedural macro will iterate over all the fields in a Rust struct and generates
/// a function that prints them all.
#[proc_macro_derive(Reflection)]
pub fn reflect_struct_fields(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the token stream into Rust AST using the `syn` crate.
    let input = parse_macro_input!(ts as DeriveInput);
    let ident = input.ident;

    // There are three types for `syn::Data`: struct, enum or union.
    if let Data::Struct(inner) = input.data {
        let mut inner_fields_quote = vec![];
        for name in inner.fields.iter() {
            if let Some(ref ident) = name.ident {
                let ident = ident.to_string();

                // An array of token streams can be expanded by quote.
                inner_fields_quote.push(quote! {
                    println!("This field name is {}", #ident);
                });
            }
        }

        let q = quote! {
            impl #ident {
                /// This function prints all the fields within a Rust struct.
                pub fn traverse_fields(&self) {
                    #(#inner_fields_quote)*
                }
            }
        };

        q.into()
    } else {
        proc_macro::TokenStream::new()
    }
}

#[proc_macro_attribute]
pub fn attribute_play(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Item);
    let args = args
        .to_string()
        .split(", ")
        .map(|s| {
            // Prevent quote to treat identifiers as pure strings (which generate quotation marks).
            let ident = format_ident!("{s}");
            quote! { #ident: u32, }
        })
        .collect::<Vec<_>>();

    if let Item::Fn(function) = input {
        let func_name = function.sig.ident;

        let q = quote! {
            pub fn #func_name(#(#args)*) {
                println!("New function!");
            }
        };

        q.into()
    } else {
        let q = quote! {
            pub fn foo() {
                println!("default!");
            }
        };

        q.into()
    }
}
