//!
//! # macro
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{env, mem::replace, ops::Deref};
use syn::{
    parse::Parse, parse_macro_input, parse_quote, punctuated::Punctuated, Fields, FnArg, ImplItem,
    ItemImpl, ItemStruct, Lit, MetaNameValue, Token, Type,
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

            fn from_abci_event(&mut self, e: abcf::abci::Event) -> abcf::Result<()> {
                Ok(())
            }

            fn from_abci_event_string(&mut self, str: String) -> abcf::Result<()> {
                let event = serde_json::from_str::<#struct_name>(&str)?;
                *self = event;
                Ok(())
            }
        }
    };

    TokenStream::from(expanded)
}

#[derive(Debug)]
struct RpcsPunctuatedMetaNameValue {
    pub module: Lit,
}

impl Parse for RpcsPunctuatedMetaNameValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut module = None;

        for mnv in parsed {
            let key = mnv
                .path
                .get_ident()
                .ok_or(input.error("no attr key"))?
                .to_string();
            match key.as_str() {
                "module" => module = Some(mnv.lit),
                _ => return Err(input.error(format_args!("key: {} no support", key))),
            }
        }

        Ok(Self {
            module: module.ok_or(input.error("module must set"))?,
        })
    }
}

///
///  Distribute the user-defined functions in the call function as a mapping
///
#[proc_macro_attribute]
pub fn rpcs(args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemImpl);
    let args = parse_macro_input!(args as RpcsPunctuatedMetaNameValue);
    println!("{:#?}", args);
    let struct_name = parsed.self_ty.clone();
    let name = match struct_name.as_ref() {
        Type::Path(path) => path.path.segments[0].ident.clone().to_string(),
        _ => "Error".to_string(),
    };

    let mut fn_names = vec![];
    let mut param_names = vec![];
    let mut param_idents = vec![];
    let module_name = match args.module {
        Lit::Str(s) => s.value(),
        _ => "".to_string(),
    };
    let mut fn_idents = vec![];
    let mut is_empty_impl = true;

    parsed.items.iter().for_each(|item| match item {
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
                            param_names.push(param_ident.to_string());
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

    let out_dir_str = env::var("OUT_DIR").expect("please create build.rs");
    let out_dir = Path::new(&out_dir_str).join(name + ".rs");
    let mut f = File::create(&out_dir).expect("create file error");

    fn_names
        .iter()
        .zip(param_names)
        .for_each(|(fn_name, param_name)| {
            let s = format!(
                r#"
                use serde_json::Value;
                use abcf_sdk::jsonrpc::{{Request, Response, endpoint}};
                use abcf_sdk::error::*;
                use abcf_sdk::providers::Provider;

                pub async fn {}<P:Provider>(param:{},mut p:P) -> Result<Option<Value>>{{
                    let data = param.as_str().unwrap().to_string();
                    let abci_query_req = endpoint::abci_query::Request{{
                        path: "rpc/{}/{}".to_string(),
                        data,
                        height:Some("0".to_string()),
                        prove: false,
                    }};
                    let req = Request::new_to_str("abci_query", abci_query_req);
                    let resp = p.request("abci_query",req.as_str()).await?;
                    return if let Some(val) = resp {{
                        let json = serde_json::from_str::<Value>(&val)?;
                        Ok(Some(json))
                    }} else {{
                        Ok(None)
                    }}
                }}
            "#,
                fn_name, param_name, module_name, fn_name
            );
            f.write_all(s.as_bytes()).expect("write error");
        });

    let expanded = if is_empty_impl {
        quote! {
            #[async_trait::async_trait]
            impl<S, D> abcf::RPCs<
                <Self as abcf::manager::ModuleStorage>::Stateless,
                <Self as abcf::manager::ModuleStorage>::Stateful
            > for #struct_name<S, D>
            where
                S: abcf::bs3::Store + 'static,
                D: abcf::digest::Digest + Send + Sync,
            {
                async fn call(
                    &mut self,
                    ctx: &mut abcf::manager::RContext<
                        <Self as abcf::manager::ModuleStorage>::Stateless,
                        <Self as abcf::manager::ModuleStorage>::Stateful
                    >,
                    method: &str,
                    params: serde_json::Value)
                -> abcf::Result<Option<serde_json::Value>> {
                    Ok(None)
                }
            }
        }
    } else {
        quote! {
            #parsed

            #[async_trait::async_trait]
            impl<S, D> abcf::RPCs<
                <Self as abcf::manager::ModuleStorage>::Stateless,
                <Self as abcf::manager::ModuleStorage>::Stateful
            > for #struct_name<S, D>
            where
                S: abcf::bs3::Store + 'static,
                D: abcf::digest::Digest,
            {
                async fn call(
                    &mut self,
                    ctx: &mut abcf::manager::RContext<
                        <Self as abcf::manager::ModuleStorage>::Stateless,
                        <Self as abcf::manager::ModuleStorage>::Stateful
                    >,
                    method: &str,
                    params: serde_json::Value)
                -> abcf::Result<Option<serde_json::Value>> {
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
    let mut parsed = parse_macro_input!(input as ItemStruct);

    let struct_ident = parsed.ident.clone();
    let name = args.name;
    let version = args.version;
    let impl_version = args.impl_version;
    let target_height = args.target_height;

    let mut stateless = Vec::new();
    let mut stateless_arg = Vec::new();
    let mut stateless_value = Vec::new();
    let mut stateless_tx = Vec::new();
    let mut stateful = Vec::new();
    let mut stateful_arg = Vec::new();
    let mut stateful_tx = Vec::new();
    let mut stateful_value = Vec::new();

    if let Fields::Named(fields) = &mut parsed.fields {
        let origin_fields = replace(&mut fields.named, Punctuated::new());

        for field in origin_fields {
            let mut f = field;
            let mut is_memory = true;
            let attrs = replace(&mut f.attrs, Vec::new());
            for attr in attrs {
                if attr.path.is_ident("stateless") {
                    let mut target_field = f.clone();

                    stateless_value.push(target_field.clone());

                    let origin_ty = f.ty.clone();
                    stateless_arg.push(target_field.ident.clone().unwrap());
                    target_field.ty = parse_quote!(abcf::bs3::SnapshotableStorage<S, #origin_ty>);
                    stateless.push(target_field.clone());

                    target_field.ty = parse_quote!(abcf::bs3::Transaction<'a, S, #origin_ty>);
                    stateless_tx.push(target_field);

                    is_memory = false;
                } else if attr.path.is_ident("stateful") {
                    let mut target_field = f.clone();

                    stateful_value.push(target_field.clone());

                    let origin_ty = f.ty.clone();
                    stateful_arg.push(target_field.ident.clone().unwrap());
                    target_field.ty = parse_quote!(abcf::bs3::SnapshotableStorage<S, #origin_ty>);
                    stateful.push(target_field.clone());

                    target_field.ty = parse_quote!(abcf::bs3::Transaction<'a, S, #origin_ty>);
                    stateful_tx.push(target_field);

                    is_memory = false;
                }
            }
            if is_memory {
                fields.named.push(f.clone());
            }
        }
    }

    let stateless_struct_ident = Ident::new(
        &format!("ABCFModule{}Sl", parsed.ident.to_string()),
        Span::call_site(),
    );

    let stateless_tx_struct_ident = Ident::new(
        &format!("ABCFModule{}SlTx", parsed.ident.to_string()),
        Span::call_site(),
    );

    let stateless_tx_cache_struct_ident = Ident::new(
        &format!("ABCFModule{}SlTxCache", parsed.ident.to_string()),
        Span::call_site(),
    );

    let stateful_struct_ident = Ident::new(
        &format!("ABCFModule{}Sf", parsed.ident.to_string()),
        Span::call_site(),
    );

    let stateful_tx_struct_ident = Ident::new(
        &format!("ABCFModule{}SfTx", parsed.ident.to_string()),
        Span::call_site(),
    );

    let stateful_tx_cache_struct_ident = Ident::new(
        &format!("ABCFModule{}SfTxCache", parsed.ident.to_string()),
        Span::call_site(),
    );

    let storage_module_ident = Ident::new(
        &format!("__abcf_storage_{}", parsed.ident.to_string().to_lowercase()),
        Span::call_site(),
    );

    let module_name = parsed.ident.clone();
    let memory_fileds = if let Fields::Named(fields) = parsed.fields {
        fields.named.iter().map(|e| e.clone()).collect()
    } else {
        Vec::new()
    };

    let result = quote! {
        pub struct #module_name<S, D>
        where
            S: abcf::bs3::Store + 'static,
            D: abcf::digest::Digest,
        {
            #(
                #memory_fileds,
            )*
            __marker_s: core::marker::PhantomData<S>,
            __marker_d: core::marker::PhantomData<D>,
        }

        impl<S, D> abcf::manager::ModuleStorage for #module_name<S, D>
        where
            S: abcf::bs3::Store + 'static,
            D: abcf::digest::Digest,
        {
            type Stateless = #storage_module_ident::#stateless_struct_ident<S>;
            type Stateful = #storage_module_ident::#stateful_struct_ident<S>;
        }

        impl<S, D> abcf::Module for #struct_ident<S, D>
        where
            S: abcf::bs3::Store,
            D: abcf::digest::Digest,
        {
            fn metadata(&self) -> abcf::ModuleMetadata<'_> {
                abcf::ModuleMetadata {
                    name: #name,
                    version: #version,
                    impl_version: #impl_version,
                    module_type: abcf::ModuleType::Module,
                    genesis: abcf::Genesis {
                        target_height: #target_height,
                    }
                }
            }
        }

        mod #storage_module_ident {
            use super::*;
            use abcf::Result;
            pub struct #stateless_struct_ident<S>
            where
                S: abcf::bs3::Store,
            {
                #(
                    #stateless,
                )*
            }

            pub struct #stateless_tx_struct_ident<'a, S>
            where
                S: abcf::bs3::Store,
            {
                #(
                    #stateless_tx,
                )*
            }

            pub struct #stateless_tx_cache_struct_ident {
                #(
                    #stateless_value,
                )*
            }

            impl<S> abcf::Storage for #stateless_struct_ident<S>
            where
                S: abcf::bs3::Store,
            {
                fn rollback(&mut self, height: i64) -> Result<()> {
                    #(
                        self.#stateless_arg.rollback(height)?;
                    )*
                    Ok(())
                }

                fn height(&self) -> Result<i64> {
                    Ok(0)
                }

                fn commit(&mut self) -> Result<()> {
                    #(
                        self.#stateless_arg.commit()?;
                    )*
                    Ok(())
                }
            }

            impl<S> abcf::module::StorageTransaction for #stateless_struct_ident<S>
            where
                S: abcf::bs3::Store + 'static,
            {
                type Transaction<'a> = #stateless_tx_struct_ident<'a, S>;

                type Cache = #stateless_tx_cache_struct_ident;

                fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
                    Self::Cache {
                        #(
                            #stateless_arg: tx.#stateless_arg.value,
                        )*
                    }
                }

                fn transaction(&self) -> Self::Transaction<'_> {
                    #stateless_tx_struct_ident {
                        #(
                            #stateless_arg: self.#stateless_arg.transaction(),
                        )*
                    }
                }

                fn execute(&mut self, transaction: Self::Cache) {

                }
            }

            impl<S, D> abcf::module::Merkle<D> for #stateless_struct_ident<S>
            where
                S: abcf::bs3::Store,
                D: abcf::digest::Digest,
            {
                fn root(&self) -> Result<abcf::digest::Output<D>> {
                    Ok(Default::default())
                }
            }

            pub struct #stateful_struct_ident<S>
            where
                S: abcf::bs3::Store,
            {
                #(
                    #stateful,
                )*
            }
            pub struct #stateful_tx_struct_ident<'a, S>
            where
                S: abcf::bs3::Store,
            {
                #(
                    #stateful_tx,
                )*
            }

            pub struct #stateful_tx_cache_struct_ident {
                #(
                    #stateful_value,
                )*
            }

            impl<S> abcf::Storage for #stateful_struct_ident<S>
            where
                S: abcf::bs3::Store,
            {
                fn rollback(&mut self, height: i64) -> Result<()> {
                    #(
                        self.#stateful_arg.rollback(height)?;
                    )*
                    Ok(())
                }

                fn height(&self) -> Result<i64> {
                    Ok(0)
                }

                fn commit(&mut self) -> Result<()> {
                    #(
                        self.#stateful_arg.commit()?;
                    )*
                    Ok(())
                }
            }

            impl<S> abcf::module::StorageTransaction for #stateful_struct_ident<S>
            where
                S: abcf::bs3::Store + 'static,
            {
                type Transaction<'a> = #stateful_tx_struct_ident<'a, S>;

                type Cache = #stateful_tx_cache_struct_ident;

                fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
                    Self::Cache {
                        #(
                            #stateful_arg: tx.#stateful_arg.value,
                        )*
                    }
                }

                fn transaction(&self) -> Self::Transaction<'_> {
                    #stateful_tx_struct_ident {
                        #(
                            #stateful_arg: self.#stateful_arg.transaction(),
                        )*
                    }
                }

                fn execute(&mut self, transaction: Self::Cache) {

                }
            }

            impl<S, D> abcf::module::Merkle<D> for #stateful_struct_ident<S>
            where
                S: abcf::bs3::Store,
                D: abcf::digest::Digest,
            {
                fn root(&self) -> Result<abcf::digest::Output<D>> {
                    Ok(Default::default())
                }
            }


        }
    };

    TokenStream::from(result)
}

#[proc_macro_attribute]
pub fn application(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemImpl);

    let module_name = parsed.self_ty;

    let inner = parsed.items;

    let trait_name = if let Some(t) = parsed.trait_ {
        t.1
    } else {
        parse_quote!(abcf::Application)
    };

    let result = quote! {
        #[async_trait::async_trait]
        impl<S, D> #trait_name<abcf::Stateless<Self>, abcf::Stateful<Self>> for #module_name<S, D>
        where
            S: abcf::bs3::Store,
            D: abcf::digest::Digest + Sync + Send,
        {
            #(
                #inner
            )*
        }
    };
    TokenStream::from(result)
}

#[proc_macro_attribute]
pub fn methods(_args: TokenStream, input: TokenStream) -> TokenStream {
    let parsed = parse_macro_input!(input as ItemImpl);

    let module_name = parsed.self_ty;

    let inner = parsed.items;

    let result = quote! {
        impl<S, D> #module_name<S, D>
        where
            S: abcf::bs3::Store,
            D: abcf::digest::Digest + Sync + Send,
        {
            #(
                #inner
            )*
        }
    };
    TokenStream::from(result)
}
