//!
//! # macro
//!

#![deny(warnings)]
#![deny(missing_docs)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use std::ops::Deref;
use syn::{parse_macro_input, FnArg, ImplItem, ItemImpl, ItemStruct, Type};

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
            fn to_abci_event(&self) -> abcf::Result<tm_protos::abci::Event> {

                let mut attributes = Vec::new();

                #(
                    let key_byte = #key_str_vec.as_bytes().to_vec();

                    if let Ok(value_byte) = serde_json::to_vec(&self.#key_vec) {
                        let index = #index_vec;

                        let a = tm_protos::abci::EventAttribute{
                            key: key_byte,
                            value: value_byte,
                            index,
                        };
                        attributes.push(a);
                    } else {
                        return Err(abcf::Error::JsonParseError)
                    }

                )*

                Ok(abci::Event {
                    r#type: self.name().to_string(),
                    attributes,
                })
            }

            fn name(&self) -> &str {
                #name
            }
        }
    };

    TokenStream::from(expanded)
}

///
///  Distribute the user-defined functions in the call function as a mapping
///
#[proc_macro_attribute]
pub fn rpcs(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemImpl);
    let struct_name = parsed.self_ty.clone();

    let mut fn_names = vec![];
    let mut fn_idents = vec![];
    let mut param_idents = vec![];

    parsed.items.iter().for_each(|item| match item {
        ImplItem::Const(_) => {}
        ImplItem::Method(data) => {
            let fn_name = data.sig.ident.clone().to_string();
            fn_names.push(fn_name);
            fn_idents.push(data.sig.ident.clone());
            data.sig.inputs.iter().for_each(|input| match input {
                FnArg::Receiver(_) => {}
                FnArg::Typed(typed) => match typed.ty.deref() {
                    Type::Path(p) => {
                        p.path.segments.iter().for_each(|seg| {
                            let param_ident = seg.ident.clone();
                            param_idents.push(param_ident);
                        });
                    }
                    _ => {}
                },
            });
        }
        _ => {}
    });

    let expanded = quote! {
        #parsed

        #[async_trait::async_trait]
        impl abcf::RPCs for #struct_name {
            async fn call(&mut self, ctx: &mut abcf::abci::Context, method: &str, params: serde_json::Value) -> abcf::RPCResponse<'_, serde_json::Value> {

                return if let Ok(resp) = match method {
                    #(
                        #fn_names
                    )* => {#(
                        let param = serde_json::from_value::<#param_idents>(params).unwrap();

                        if let Ok(resp) = self.#fn_idents(ctx,param).await {
                            if let Ok(v) = serde_json::to_value(resp){
                                abcf::Result::Ok(v)
                            } else {
                                Err(abcf::Error::RPRApplicationError(10005,"call rpc error".to_string()))
                            }
                        } else {
                            Err(abcf::Error::JsonParseError)
                        }


                    )*}
                    _ => {Err(abcf::Error::TempOnlySupportRPC)}
                } {
                    RPCResponse{
                        code:0,
                        message:"success",
                        data:Some(resp),
                    }
                } else {
                    RPCResponse{
                        code:1,
                        message:"failed",
                        data:None,
                    }
                }
            }
        }
    };
    TokenStream::from(expanded)
}
