use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    Arm, FieldValue, Fields, FnArg, GenericParam, ItemImpl, ItemStruct, Lit, LitStr, MetaNameValue,
    PathArguments, Token, Type,
};

use crate::utils::ParseField;

struct ManagerMetaInfo {
    pub name: Lit,
    pub digest: LitStr,
    pub transaction: LitStr,
    pub version: Lit,
    pub impl_version: Lit,
}

impl Parse for ManagerMetaInfo {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut name = None;
        let mut digest = None;
        let mut transaction = None;
        let mut version = None;
        let mut impl_version = None;

        let parsed = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        for meta in parsed {
            let key = meta
                .path
                .get_ident()
                .ok_or(input.error("no attr key"))?
                .to_string();
            match key.as_str() {
                "name" => name = Some(meta.lit),
                "digest" => {
                    digest = match meta.lit {
                        Lit::Str(s) => Some(s),
                        _ => None,
                    }
                }
                "transaction" => {
                    transaction = match meta.lit {
                        Lit::Str(s) => Some(s),
                        _ => None,
                    }
                }
                "version" => version = Some(meta.lit),
                "impl_version" => impl_version = Some(meta.lit),
                _ => return Err(input.error(format_args!("key: {} no support", key))),
            }
        }

        Ok(Self {
            name: name.ok_or(input.error("name must set"))?,
            digest: digest.ok_or(input.error("digest must set"))?,
            transaction: transaction.ok_or(input.error("digest must set"))?,
            version: version.ok_or(input.error("verison must set"))?,
            impl_version: impl_version.ok_or(input.error("impl_version must set"))?,
        })
    }
}

pub fn manager(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as ManagerMetaInfo);
    let mut parsed = parse_macro_input!(input as ItemStruct);

    let digest = args
        .digest
        .parse_with(syn::Path::parse_mod_style)
        .expect("digest must a path");
    let transaction = args
        .transaction
        .parse_with(syn::Path::parse_mod_style)
        .expect("transaction must a path");
    let name = args.name;
    let module_name = parsed.ident.clone();
    let version = args.version;
    let impl_version = args.impl_version;

    // add <S> on module
    for item in &mut parsed.fields {
        if let Type::Path(p) = &mut item.ty {
            let segments = &mut p.path.segments;
            let arguments = &mut segments.last_mut().unwrap().arguments;
            if let PathArguments::AngleBracketed(a) = arguments {
                a.args.push(parse_quote!(S));
            } else {
                *arguments = PathArguments::AngleBracketed(parse_quote!(<S>));
            }
        }
    }

    let mut init_items = Vec::new();
    let mut fn_items = Vec::new();
    let mut stateless_struct_items = Vec::new();
    let mut stateful_struct_items = Vec::new();
    let mut tree_match_arms = Vec::new();
    let mut key_item = Vec::new();

    let mut sl_tx_items = Vec::new();
    let mut sl_cache_items = Vec::new();

    let mut sf_tx_items = Vec::new();
    let mut sf_cache_items = Vec::new();

    let mut sl_cache_init_items = Vec::new();
    let mut sf_cache_init_items = Vec::new();

    let mut rpc_match_arms = Vec::new();

    // list items.
    for item in &mut parsed.fields {
        let key = item.ident.as_ref().expect("module must a named struct");
        let ty = &item.ty;
        let name_lit_str = LitStr::new(&key.to_string(), Span::call_site());

        key_item.push(key.clone());

        let fv: FieldValue = parse_quote!(#key);
        init_items.push(fv);

        let fa: FnArg = parse_quote!(#key: #ty);
        fn_items.push(fa);

        let sl_struct_item: ParseField = parse_quote!(pub #key: abcf::Stateless<#ty>);
        stateless_struct_items.push(sl_struct_item);

        let sf_struct_item: ParseField = parse_quote!(pub #key: abcf::Stateful<#ty>);
        stateful_struct_items.push(sf_struct_item);

        let tree_arm: Arm = parse_quote!(#name_lit_str => Ok(self.#key.get(key, height)?));
        tree_match_arms.push(tree_arm);

        let sl_tx_item: ParseField = parse_quote!(#key: abcf::StatelessBatch<'a, #ty>);
        sl_tx_items.push(sl_tx_item);

        let sf_tx_item: ParseField = parse_quote!(#key: abcf::StatefulBatch<'a, #ty>);
        sf_tx_items.push(sf_tx_item);

        let sl_cache_item: ParseField = parse_quote!(#key: abcf::StatelessCache<#ty>);
        sl_cache_items.push(sl_cache_item);

        let sf_cache_item: ParseField = parse_quote!(#key: abcf::StatefulCache<#ty>);
        sf_cache_items.push(sf_cache_item);

        let slcii: FieldValue = parse_quote!(#key: abcf::Stateless::<#ty>::cache(tx.#key));
        sl_cache_init_items.push(slcii);

        let sfcii: FieldValue = parse_quote!(#key: abcf::Stateful::<#ty>::cache(tx.#key));
        sf_cache_init_items.push(sfcii);

        let rma: Arm = parse_quote! {
            #name_lit_str => {
                let mut context = abcf::manager::RContext {
                    stateful: &ctx.stateful.#key,
                    stateless: &mut ctx.stateless.#key,
                };

                self.#key
                    .call(&mut context, method, params)
                    .await
                    .map_err(|e| abcf::ModuleError {
                        namespace: String::from(#name_lit_str),
                        error: e,
                    })
            }
        };
        rpc_match_arms.push(rma);
    }

    // add <S> on manager.
    let backked_s: ParseField = parse_quote!(__marker_s: core::marker::PhantomData<S>);
    if let Fields::Named(fields) = &mut parsed.fields {
        fields.named.push(backked_s.inner.clone());
    };

    //     stateless_struct_items.push(backked_s.clone());
    //     stateful_struct_items.push(backked_s.clone());

    parsed
        .generics
        .params
        .push(parse_quote!(S: abcf::bs3::Store + 'static));

    let mut generics_names = Vec::new();
    let mut lifetime_names = Vec::new();

    for x in &parsed.generics.params {
        if let GenericParam::Type(t) = x {
            generics_names.push(t.ident.clone());
        } else if let GenericParam::Lifetime(l) = x {
            lifetime_names.push(l.lifetime.clone());
        }
    }

    // add how to new manager.
    let mut new_impl: ItemImpl = parse_quote! {
        impl #module_name<#(#lifetime_names,)* #(#generics_names,)*> {
            pub fn new(#(#fn_items,)*) -> Self {
                Self {
                    #(#init_items,)*
                    __marker_s: core::marker::PhantomData,
                }
            }
        }
    };
    new_impl.generics = parsed.generics.clone();

    // add metadata trait define.
    let mut metadata_trait: ItemImpl = parse_quote! {
        impl abcf::Module for #module_name<#(#lifetime_names,)* #(#generics_names,)*> {
            fn metadata(&self) -> abcf::ModuleMetadata<'_> {
                abcf::ModuleMetadata {
                    name: #name,
                    version: #version,
                    impl_version: #impl_version,
                    module_type: abcf::ModuleType::Manager,
                    genesis: abcf::Genesis {
                        target_height: 0,
                    }
                }
            }
        }
    };

    metadata_trait.generics = parsed.generics.clone();

    // group of manager.
    let stateless_struct_ident = Ident::new(
        &format!("ABCFManager{}Sl", parsed.ident.to_string()),
        Span::call_site(),
    );

    let mut stateless_struct: ItemStruct = parse_quote! {
        pub struct #stateless_struct_ident {
            #(#stateless_struct_items,)*
        }
    };
    stateless_struct.generics = parsed.generics.clone();

    let mut stateless_struct_tree: ItemImpl = parse_quote! {
        impl abcf::entry::Tree for #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn get(&self, key: &str, height: i64) -> abcf::ModuleResult<Vec<u8>> {
                let mut splited = key.splitn(2, "/");

                let module_name = splited.next().ok_or(abcf::ModuleError {
                    namespace: String::from(#name),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                let inner_key = splited.next().ok_or(abcf::ModuleError {
                    namespace: String::from(#name),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                match module_name {
                    #(#tree_match_arms,)*
                    _ => Err(abcf::ModuleError {
                        namespace: String::from(#name),
                        error: abcf::Error::NoModule,
                    })
                }
            }
        }
    };
    stateless_struct_tree.generics = parsed.generics.clone();

    let sl_tx_struct_ident = Ident::new(
        &format!("ABCFManager{}SlTx", parsed.ident.to_string()),
        Span::call_site(),
    );

    let mut sl_tx: ItemStruct = parse_quote! {
        pub struct #sl_tx_struct_ident {
            #(#sl_tx_items,)*
        }
    };

    sl_tx.generics = parsed.generics.clone();
    sl_tx.generics.params.push(parse_quote!('a));

    let sl_tx_cache_struct_ident = Ident::new(
        &format!("ABCFManager{}SlTxCache", parsed.ident.to_string()),
        Span::call_site(),
    );

    let mut sl_cache: ItemStruct = parse_quote! {
        pub struct #sl_tx_cache_struct_ident {
            #(#sl_cache_items,)*
        }
    };
    sl_cache.generics = parsed.generics.clone();

    // TODO: add store for manager, use this store to force height.
    let __module1 = key_item.first().unwrap();

    let mut sl_storage_impl: ItemImpl = parse_quote! {
        impl abcf::Storage for #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn rollback(&mut self, height: i64) -> Result<()> {
                #(
                    self.#key_item.rollback(height)?;
                )*
                Ok(())
            }

            fn height(&self) -> Result<i64> {
                Ok(self.#__module1.height()?)
            }

            fn commit(&mut self) -> Result<()> {
                #(
                    self.#key_item.commit()?;
                )*
                Ok(())
            }
        }
    };
    sl_storage_impl.generics = parsed.generics.clone();

    let mut sl_storage_tx_impl: ItemImpl = parse_quote! {
        impl abcf::module::StorageTransaction for #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            type Transaction<'a> = #sl_tx_struct_ident<'a, #(#lifetime_names,)* #(#generics_names,)*>;

            type Cache = #sl_tx_cache_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>;

            fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
                Self::Cache {
                    #(
                        #sl_cache_init_items,
                    )*
                }
            }

            fn transaction(&self) -> Self::Transaction<'_> {
                Self::Transaction::<'_> {
                    mock: self.mock.transaction(),
                    mock2: self.mock2.transaction(),
                }
            }

            fn execute(&mut self, transaction: Self::Cache) {
                self.mock.execute(transaction.mock);
                self.mock.execute(transaction.mock2);
            }
        }
    };
    sl_storage_tx_impl.generics = parsed.generics.clone();

    // stateful define

    let stateful_struct_ident = Ident::new(
        &format!("ABCFManager{}Sf", parsed.ident.to_string()),
        Span::call_site(),
    );

    let mut stateful_struct: ItemStruct = parse_quote! {
        pub struct #stateful_struct_ident {
            #(#stateful_struct_items,)*
        }
    };
    stateful_struct.generics = parsed.generics.clone();

    let mut stateful_struct_tree: ItemImpl = parse_quote! {
        impl abcf::entry::Tree for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn get(&self, key: &str, height: i64) -> abcf::ModuleResult<Vec<u8>> {
                let mut splited = key.splitn(2, "/");

                let module_name = splited.next().ok_or(abcf::ModuleError {
                    namespace: String::from(#name),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                let inner_key = splited.next().ok_or(abcf::ModuleError {
                    namespace: String::from(#name),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                match module_name {
                    #(#tree_match_arms,)*
                    _ => Err(abcf::ModuleError {
                        namespace: String::from(#name),
                        error: abcf::Error::NoModule,
                    })
                }
            }
        }
    };
    stateful_struct_tree.generics = parsed.generics.clone();

    let mut stateful_merkle: ItemImpl = parse_quote! {
        impl<S> abcf::module::Merkle<#digest> for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>
        where
            S: abcf::bs3::Store,
        {
            fn root(&self) -> abcf::Result<digest::Output<Sha3_512>> {
                use digest::Digest;
                use abcf::module::Merkle;

                let mut hasher = #digest::new();
                #(
                    {
                        let item = &self.#key_item as &dyn Merkle<#digest>;
                        hasher.update(item.root()?);
                    }
                )*
                Ok(hasher.finalize())
            }
        }
    };
    stateful_merkle.generics = parsed.generics.clone();

    let sf_tx_struct_ident = Ident::new(
        &format!("ABCFManager{}SfTx", parsed.ident.to_string()),
        Span::call_site(),
    );

    let mut sf_tx: ItemStruct = parse_quote! {
        pub struct #sf_tx_struct_ident {
            #(#sf_tx_items,)*
        }
    };

    sf_tx.generics = parsed.generics.clone();
    sf_tx.generics.params.push(parse_quote!('a));

    let sf_tx_cache_struct_ident = Ident::new(
        &format!("ABCFManager{}SfTxCache", parsed.ident.to_string()),
        Span::call_site(),
    );

    let mut sf_cache: ItemStruct = parse_quote! {
        pub struct #sf_tx_cache_struct_ident {
            #(#sf_cache_items,)*
        }
    };

    sf_cache.generics = parsed.generics.clone();
    let storage_module_ident = Ident::new(
        &format!("__abcf_storage_{}", parsed.ident.to_string().to_lowercase()),
        Span::call_site(),
    );

    let mut store_trait: ItemImpl = parse_quote! {
        impl abcf::manager::ModuleStorage for #module_name<#(#lifetime_names,)* #(#generics_names,)*> {
            type Stateless = #storage_module_ident::#stateless_struct_ident<S>;
            type Stateful = #storage_module_ident::#stateful_struct_ident<S>;
        }
    };

    store_trait.generics = parsed.generics.clone();

    // TODO: add store for manager, use this store to force height.
    let mut sf_storage_impl: ItemImpl = parse_quote! {
        impl abcf::Storage for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn rollback(&mut self, height: i64) -> Result<()> {
                #(
                    self.#key_item.rollback(height)?;
                )*
                Ok(())
            }

            fn height(&self) -> Result<i64> {
                Ok(self.#__module1.height()?)
            }

            fn commit(&mut self) -> Result<()> {
                #(
                    self.#key_item.commit()?;
                )*
                Ok(())
            }
        }
    };
    sf_storage_impl.generics = parsed.generics.clone();

    let mut sf_storage_tx_impl: ItemImpl = parse_quote! {
        impl abcf::module::StorageTransaction for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            type Transaction<'a> = #sf_tx_struct_ident<'a, #(#lifetime_names,)* #(#generics_names,)*>;

            type Cache = #sf_tx_cache_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>;

            fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
                Self::Cache {
                    #(
                        #sf_cache_init_items,
                    )*
                }
            }

            fn transaction(&self) -> Self::Transaction<'_> {
                Self::Transaction::<'_> {
                    mock: self.mock.transaction(),
                    mock2: self.mock2.transaction(),
                }
            }

            fn execute(&mut self, transaction: Self::Cache) {
                self.mock.execute(transaction.mock);
                self.mock.execute(transaction.mock2);
            }
        }
    };
    sf_storage_tx_impl.generics = parsed.generics.clone();

    let mut app_impl: ItemImpl = parse_quote! {
            #[async_trait::async_trait]
            impl abcf::entry::Application<
                #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>,
                #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>
            >
            for #module_name<#(#lifetime_names,)* #(#generics_names,)*>
            {
                async fn check_tx(
                    &mut self,
                    context: &mut abcf::entry::TContext<
                        #sl_tx_struct_ident<'_, #(#lifetime_names,)* #(#generics_names,)*>,
                        #sf_tx_struct_ident<'_, #(#lifetime_names,)* #(#generics_names,)*>,
                    >,
                    _req: abcf::abci::RequestCheckTx,
                ) -> abcf::ModuleResult<abcf::module::types::ResponseCheckTx> {
                    use abcf::module::FromBytes;
                    use std::collections::BTreeMap;
                    use abcf::Module;
                    use abcf::Error;

                    let req_tx =
                        SimpleNodeTransaction::from_bytes(&_req.tx).map_err(|e| abcf::ModuleError {
                            namespace: String::from("abcf.manager"),
                            error: e,
                        })?;

                    // let mut ctx = abcf::manager::TContext {
                    //     events: abcf::entry::EventContext {
                    //         events: context.events.events,
                    //     },
                    //     stateful: &mut context.stateful.mock,
                    //     stateless: &mut context.stateless.mock,
                    // };

                    let tx = abcf::module::types::RequestCheckTx {
                        ty: _req.r#type,
                        tx: req_tx.into(),
                    };

                    // let result = self
                    //     .mock
                    //     .check_tx(&mut ctx, &tx)
                    //     .await
                    //     .map_err(|e| abcf::ModuleError {
                    //         namespace: String::from("mock"),
                    //         error: e,
                    //     })?;

                    let mut resp_check_tx = abcf::module::types::ResponseCheckTx::default();
                    let mut data_map = BTreeMap::new();

                    #(
                        let mut ctx = abcf::manager::TContext {
                            events: abcf::entry::EventContext {
                                events: context.events.events,
                            },
                            stateful: &mut context.stateful.#key_item,
                            stateless: &mut context.stateless.#key_item,
                        };
                        let name = self.#key_item.metadata().name.to_string();
                        let result = self.#key_item
                            .check_tx(&mut ctx, &tx)
                            .await
                            .map_err(|e| abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            })?;

                        data_map.insert(name.clone(), result.data);
                        resp_check_tx.gas_used += result.gas_used;
                        resp_check_tx.gas_wanted += result.gas_wanted;

                    )*
                    let data = serde_json::to_vec(&data_map).map_err(|e|abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: Error::JsonError(e),
                            })?;
                    resp_check_tx.data = data;

                    Ok(resp_check_tx)
                }

    //             /// Begin block.
            //     async fn begin_block(
            //         &mut self,
            //         context: &mut abcf::entry::AContext<SimpleNodeSl<S>, SimpleNodeSf<S>>,
            //         _req: abcf::module::types::RequestBeginBlock,
            //     ) {
            //         let mut ctx = abcf::manager::AContext {
            //             events: abcf::entry::EventContext {
            //                 events: context.events.events,
            //             },
            //             stateful: &mut context.stateful.mock,
            //             stateless: &mut context.stateless.mock,
            //         };
            //
            //         self.mock.begin_block(&mut ctx, &_req).await;
            //     }
            //
            //     /// Execute transaction on state.
            //     async fn deliver_tx(
            //         &mut self,
            //         context: &mut abcf::entry::TContext<SimpleNodeSlTx<'_, S>, SimpleNodeSfTx<'_, S>>,
            //         _req: abcf::abci::RequestDeliverTx,
            //     ) -> abcf::ModuleResult<abcf::module::types::ResponseDeliverTx> {
            //         use abcf::module::FromBytes;
            //
            //         let mut ctx = abcf::manager::TContext {
            //             events: abcf::entry::EventContext {
            //                 events: context.events.events,
            //             },
            //             stateful: &mut context.stateful.mock,
            //             stateless: &mut context.stateless.mock,
            //         };
            //
            //         let req_tx =
            //             SimpleNodeTransaction::from_bytes(&_req.tx).map_err(|e| abcf::ModuleError {
            //                 namespace: String::from("mock"),
            //                 error: e,
            //             })?;
            //
            //         let tx = abcf::module::types::RequestDeliverTx { tx: req_tx.into() };
            //
            //         let result = self
            //             .mock
            //             .deliver_tx(&mut ctx, &tx)
            //             .await
            //             .map_err(|e| abcf::ModuleError {
            //                 namespace: String::from("mock"),
            //                 error: e,
            //             })?;
            //
            //         Ok(result)
            //     }
            //
            //     /// End Block.
            //     async fn end_block(
            //         &mut self,
            //         context: &mut abcf::entry::AContext<SimpleNodeSl<S>, SimpleNodeSf<S>>,
            //         _req: abcf::module::types::RequestEndBlock,
            //     ) -> abcf::module::types::ResponseEndBlock {
            //         let mut ctx = abcf::manager::AContext {
            //             events: abcf::entry::EventContext {
            //                 events: context.events.events,
            //             },
            //             stateful: &mut context.stateful.mock,
            //             stateless: &mut context.stateless.mock,
            //         };
            //
            //         self.mock.end_block(&mut ctx, &_req).await
            //     }
            // }
            //
            // #[async_trait::async_trait]
            // impl<S> abcf::entry::RPCs<SimpleNodeSl<S>, SimpleNodeSf<S>> for SimpleNode<S>
            // where
            //     S: abcf::bs3::Store + 'static,
            // {
            //     async fn call(
            //         &mut self,
            //         ctx: &mut abcf::entry::RContext<SimpleNodeSl<S>, SimpleNodeSf<S>>,
            //         method: &str,
            //         params: serde_json::Value,
            //     ) -> abcf::ModuleResult<Option<serde_json::Value>> {
            //         use abcf::RPCs;
            //         let mut paths = method.split("/");
            //         let module_name = paths.next().ok_or(abcf::ModuleError {
            //             namespace: String::from("abcf.manager"),
            //             error: abcf::Error::QueryPathFormatError,
            //         })?;
            //
            //         let method = paths.next().ok_or(abcf::ModuleError {
            //             namespace: String::from("abcf.managing"),
            //             error: abcf::Error::QueryPathFormatError,
            //         })?;
            //
            //         match module_name {
            //             "mock" => {
            //                 let mut context = abcf::manager::RContext {
            //                     stateful: &ctx.stateful.mock,
            //                     stateless: &mut ctx.stateless.mock,
            //                 };
            //
            //                 self.mock
            //                     .call(&mut context, method, params)
            //                     .await
            //                     .map_err(|e| abcf::ModuleError {
            //                         namespace: String::from("mock"),
            //                         error: e,
            //                     })
            //             }
            //             _ => Err(abcf::ModuleError {
            //                 namespace: String::from("abcf.manager"),
            //                 error: abcf::Error::NoModule,
            //             }),
            //         }
            //     }
            }
        };

    app_impl.generics = parsed.generics.clone();

    let mut rpc_impl: ItemImpl = parse_quote! {
        #[async_trait::async_trait]
        impl abcf::entry::RPCs<
            #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>,
            #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>
        >
        for #module_name<#(#lifetime_names,)* #(#generics_names,)*>
        {
            async fn call(
                &mut self,
                ctx: &mut abcf::entry::RContext<
                    #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>,
                    #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>
                >,
                method: &str,
                params: serde_json::Value,
            ) -> abcf::ModuleResult<Option<serde_json::Value>> {
                use abcf::RPCs;
                let mut paths = method.split("/");
                let module_name = paths.next().ok_or(abcf::ModuleError {
                    namespace: String::from("abcf.manager"),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                let method = paths.next().ok_or(abcf::ModuleError {
                    namespace: String::from("abcf.manager"),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                match module_name {
                    #(#rpc_match_arms)*
                    _ => Err(abcf::ModuleError {
                        namespace: String::from("abcf.manager"),
                        error: abcf::Error::NoModule,
                    }),
                }
            }
        }
    };

    rpc_impl.generics = parsed.generics.clone();

    let result = quote! {
        #parsed

        #new_impl

        #metadata_trait

        #store_trait

        pub mod #storage_module_ident {
            use super::*;
            use abcf::Result;

            pub const MODULE_NAME: &'static str = #name;

            #stateless_struct

            #stateless_struct_tree

            #sl_tx

            #sl_cache

            #sl_storage_impl

            #sl_storage_tx_impl

            #stateful_struct

            #stateful_struct_tree

            #stateful_merkle

            #sf_tx

            #sf_cache

            #sf_storage_impl

            #sf_storage_tx_impl

            #app_impl

            #rpc_impl
        }
    };

    TokenStream::from(result)
}
