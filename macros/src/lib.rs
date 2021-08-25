//!
//! # macro
//!

#![deny(warnings)]
#![deny(missing_docs)]

extern crate proc_macro;
extern crate proc_macro2;

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

    parsed.fields.iter().for_each(|f| {
        f.attrs.iter().for_each(|a| {
            a.path
                .segments
                .iter()
                .for_each(|s| match s.ident.to_string().as_str() {
                    "abcf" => {
                        index_is_false_str += &*(count.to_string() + ",");
                    }
                    _ => {}
                });
        });
        count += 1;
    });

    let expanded = quote! {

        impl abcf::Event for #struct_name {
            fn to_abci_event(&self) -> tm_protos::abci::Event {

                let mut attributes = Vec::new();
                let yy = pnk!(serde_json::to_value(self));
                yy.as_object().iter().for_each(|t|{
                    let mut count = 0;
                    t.iter().for_each(|v|{
                        let mut index = false;
                        if #index_is_false_str.contains(&*count.to_string()) {
                            index = true;
                        }
                        let key_byte = pnk!(serde_json::to_vec(v.0));
                        let value_byte = pnk!(serde_json::to_vec(v.1));
                        let a = tm_protos::abci::EventAttribute{
                            key: key_byte,
                            value: value_byte,
                            index,
                        };
                        attributes.push(a);
                        count += 1;
                    });
                });

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
