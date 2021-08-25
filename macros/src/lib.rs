//!
//! # macro
//!

#![deny(warnings)]
#![deny(missing_docs)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use syn::{parse_macro_input, ItemStruct};

///
/// Convert struct to abci::event
///
#[proc_macro_derive(Event, attributes(abcf))]
pub fn event(input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemStruct);

    let struct_name = parsed.ident.clone();
    let name = struct_name.to_string();

    let mut index_is_false_str = String::new();
    let mut count = 0;

    let mut key_vec = vec![];
    let mut key_str_vec = vec![];
    let mut index_vec = vec![];

    parsed.fields.iter().for_each(|f| {
        let mut index = false;
        f.attrs.iter().for_each(|a| {
            a.path
                .segments
                .iter()
                .for_each(|s| match s.ident.to_string().as_str() {
                    "abcf" => {
                        index_is_false_str += &*(count.to_string() + ",");
                        index = true;
                    }
                    _ => {}
                });
        });
        key_vec.push(f.ident.as_ref());
        key_str_vec.push(f.ident.clone().unwrap().to_string());
        index_vec.insert(count, index);
        count += 1;
    });


    let expanded = quote! {

        impl abcf::Event for #struct_name {
            fn to_abci_event(&self) -> tm_protos::abci::Event {

                let mut attributes = Vec::new();

                #(
                    let key_byte = serde_json::to_vec(#key_str_vec)
                            .unwrap_or_else(|e|{println!("{:?}",e);vec![]});

                    let value_byte = serde_json::to_vec(&self.#key_vec)
                            .unwrap_or_else(|e|{println!("{:?}",e);vec![]});

                    let index = #index_vec;

                    let a = tm_protos::abci::EventAttribute{
                        key: key_byte,
                        value: value_byte,
                        index,
                    };
                    attributes.push(a);
                )*

                abci::Event {
                    r#type: self.name().to_string(),
                    attributes,
                }
            }

            fn name(&self) -> &str {
                #name
            }

            fn all() -> &'static [&'static str] {
                &[]
            }
        }
    };

    TokenStream::from(expanded)
}
