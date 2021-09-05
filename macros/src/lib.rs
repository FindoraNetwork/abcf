//!
//! # macro
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::*;
use std::ops::Deref;
use syn::{
    parse::Parse, parse_macro_input, punctuated::Punctuated, FnArg, ImplItem, ItemImpl, ItemStruct,
    Lit, MetaNameValue, Token, Type,
};

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
            fn to_abci_event(&self) -> abcf::Result<abcf::abci::Event> {

                let mut attributes = Vec::new();

                #(
                    let key_byte = #key_str_vec.as_bytes().to_vec();

                    let value_byte = serde_json::to_vec(&self.#key_vec)?;
                    let index = #index_vec;

                    let a = abcf::abci::EventAttribute{
                        key: key_byte,
                        value: value_byte,
                        index,
                    };
                    attributes.push(a);

                )*

                Ok(abcf::abci::Event {
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
    let mut is_empty_impl = true;

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
            is_empty_impl = false;
        }
        _ => {}
    });

    let expanded = if is_empty_impl {
        quote! {
            #[async_trait::async_trait]
           impl abcf::RPCs for #struct_name {
               async fn call(&mut self, ctx: &mut abcf::Context, method: &str, params: serde_json::Value) ->
               abcf::Result<Option<serde_json::Value>> {
                    Ok(None)
                }
           }
        }
    } else {
        quote! {
            #parsed

            #[async_trait::async_trait]
            impl abcf::RPCs for #struct_name {
                async fn call(&mut self, ctx: &mut abcf::Context, method: &str, params: serde_json::Value) ->
                abcf::Result<Option<serde_json::Value>> {

                    match method {
                        #(
                            #fn_names
                        )* => {#(
                            let param = serde_json::from_value::<#param_idents>(params)?;

                            let response = self.#fn_idents(ctx, param).await;

                            if response.code != 0 {
                                Err(abcf::Error::new_rpc_error(response.code, response.message))
                            } else if let Some(v) = response.data {
                                Ok(Some(serde_json::to_value(v)?))
                            } else {
                                Ok(None)
                            }
                        )*}
                        _ => {Err(abcf::Error::TempOnlySupportRPC)}
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug)]
struct PunctuatedMetaNameValue {
    pub name: Lit,
    pub version: Lit,
    pub impl_version: Lit,
    pub target_height: Lit,
}

impl Parse for PunctuatedMetaNameValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut name = None;
        let mut version = None;
        let mut impl_version = None;
        let mut target_height = None;

        for mnv in parsed {
            let key = mnv
                .path
                .get_ident()
                .ok_or(input.error("no attr key"))?
                .to_string();
            match key.as_str() {
                "name" => name = Some(mnv.lit),
                "version" => version = Some(mnv.lit),
                "impl_version" => impl_version = Some(mnv.lit),
                "target_height" => target_height = Some(mnv.lit),
                _ => return Err(input.error(format_args!("key: {} no support", key))),
            }
        }

        Ok(Self {
            name: name.ok_or(input.error("name must set"))?,
            version: version.ok_or(input.error("verison must set"))?,
            impl_version: impl_version.ok_or(input.error("impl_version must set"))?,
            target_height: target_height.ok_or(input.error("target_height must set"))?,
        })
    }
}

/// Define Module
#[proc_macro_attribute]
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as PunctuatedMetaNameValue);
    let parsed = parse_macro_input!(input as ItemStruct);

    let struct_ident = parsed.ident.clone();
    let name = args.name;
    let version = args.version;
    let impl_version = args.impl_version;
    let target_height = args.target_height;

    let result = quote! {
        #parsed

        impl abcf::Module for #struct_ident {
            fn metadata(&self) -> abcf::ModuleMetadata<'_> {
                abcf::ModuleMetadata {
                    name: #name,
                    version: #version,
                    impl_version: #impl_version,
                    genesis: abcf::Genesis {
                        target_hight: #target_height,
                    }
                }
            }
        }
    };

    TokenStream::from(result)
}
