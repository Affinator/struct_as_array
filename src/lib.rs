#![crate_type = "proc-macro"]
//! A Rust library that allows you to represent the structure as an array.
//! Library works only with named structs whose fields have the same type.
//!
//! # Examples
//! Basic usage:
//! ```
//! use struct_as_array::*;
//!
//! #[derive(AsArray)]
//! struct TestStruct {
//!     t1: i32,
//!     t2: i32,
//!     t3: i32,
//! }
//!
//! let t = TestStruct {
//!     t1: 0,
//!     t2: 1,
//!     t3: 2,
//! };
//!
//! assert_eq!(t.as_array(), [&0, &1, &2]);
//! ```
//!
//! Using as an iterator:
//!
//! ```
//! use struct_as_array::*;
//!
//! #[derive(AsArray)]
//! struct TestStruct {
//!     t1: i32,
//!     t2: i32,
//!     t3: i32,
//! }
//!
//! let t = TestStruct {
//!     t1: 0,
//!     t2: 1,
//!     t3: 2,
//! };
//!
//! for i in t.as_array() {
//!     println!("{}", i);
//! }
//! ```
extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(AsArray)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input: String = input.to_string();

    let ast = syn::parse_macro_input(&input).expect("Couldn't parse item");

    let result = struct_as_array(ast);

    result
        .to_string()
        .parse()
        .expect("couldn't parse string to tokens")
}

fn struct_as_array(ast: syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;

    match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref fields)) => {
            let mut types = Vec::new();
            for field in fields {
                let ty = &field.ty;
                types.push(ty.clone());
            }

            let ty_ref = types.pop();
            match ty_ref.clone() {
                Some(ty_ref) => {
                    for ty in types {
                        if ty != ty_ref {
                            panic!("Fields in struct {} have not same types", name)
                        }
                    }
                }
                None => panic!("Struct {} have no any fields", name),
            }

            let field_names = fields.iter().map(|f| {
                let f_name = &f.ident;
                quote!(#f_name)
            });

            let prefixed_fields = field_names.map(|name| quote! { &self.#name });
            let n = prefixed_fields.len();
            let doc_comment = format!("Represent {} as array.", name);

            quote! {
                impl #name {

                    #[doc = #doc_comment]
                    fn as_array(&self) -> [&#ty_ref; #n] {
                        [#(prefixed_fields),*]
                    }
                }
            }
        }
        _ => panic!("#[derive(AsArray)] can only be used with structs"),
    }
}
