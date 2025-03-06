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
//! // Represent as array of reference
//! assert_eq!(t.as_array(), [&0, &1, &2]);
//!
//! // Convert struct to array
//! assert_eq!(t.to_array(), [0, 1, 2]);
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
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

use syn::DeriveInput;

#[proc_macro_derive(AsArray)]
pub fn as_array(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);

    proc_macro::TokenStream::from(struct_as_array(ast))
}

fn struct_as_array(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;

    match ast.data {
        syn::Data::Struct(data_struct) => {
            let fields = &data_struct.fields;
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

            let prefixed_fields_ref = field_names.clone().map(|name| quote! { &self.#name });
            let prefixed_fields = field_names.map(|name| quote! { #name });
            let prefixed_fields_clone = prefixed_fields.clone();
            let n = prefixed_fields.len();
            let doc_comment_ref = format!("Represent {} as array.", name);
            let doc_comment = format!("Convert {} to array.", name);

            quote! {
                impl #name {

                    #[doc = #doc_comment_ref]
                    fn as_array(&self) -> [&#ty_ref; #n] {
                        [#(#prefixed_fields_ref),*]
                    }

                    #[doc = #doc_comment]
                    fn to_array(self) -> [#ty_ref; #n] {
                        let #name {#(#prefixed_fields),*} = self;
                        [#(#prefixed_fields_clone),*]
                    }

                }
            }
        }
        _ => panic!("#[derive(AsArray)] can only be used with structs"),
    }
}
