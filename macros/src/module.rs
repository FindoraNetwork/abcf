use crate::utils::ParseField;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::mem::replace;
use syn::{
    parse::Parse, parse_macro_input, parse_quote, punctuated::Punctuated, Arm, Attribute,
    FieldValue, Fields, FnArg, GenericParam, ItemImpl, ItemStruct, Lit, LitStr, MetaNameValue,
    Token,
};

#[derive(Debug)]
struct FieldParsedMetaName {
    pub merkle: LitStr,
}

impl Parse for FieldParsedMetaName {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let parsed = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut merkle = None;

        for mnv in parsed {
            let key = mnv
                .path
                .get_ident()
                .ok_or(input.error("no attr key"))?
                .to_string();
            match key.as_str() {
                "merkle" => {
                    merkle = match mnv.lit {
                        Lit::Str(s) => Some(s),
                        _ => None,
                    }
                }
                _ => return Err(input.error(format_args!("key: {} no support", key))),
            }
        }

        Ok(Self {
            merkle: merkle.ok_or(input.error("name must set"))?,
        })
    }
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

pub fn build_dependence_for_module(
    store_name: &Ident,
    namespace_ident: &Ident,
    generics: &syn::Generics,
    attrs: &[Attribute],
    generics_names: &[Ident],
    lifetime_names: &[syn::Lifetime],
) -> (Vec<ItemStruct>, ItemImpl) {
    let mut result_item = Vec::new();

    let rpc_ident = Ident::new(
        &format!("ABCFDeps{}RPC", store_name.to_string()),
        Span::call_site(),
    );

    let app_ident = Ident::new(
        &format!("ABCFDeps{}App", store_name.to_string()),
        Span::call_site(),
    );

    let txn_ident = Ident::new(
        &format!("ABCFDeps{}Txn", store_name.to_string()),
        Span::call_site(),
    );

    for attr in attrs {
        if attr.path.is_ident("dependence") {
            let parser = Punctuated::<MetaNameValue, Token![,]>::parse_terminated;
            let metas = attr.parse_args_with(parser).unwrap();

            let mut r_fields = Vec::new();
            let mut a_fields = Vec::new();
            let mut t_fields = Vec::new();

            for meta in metas {
                let name = meta.path.get_ident();

                if let Lit::Str(s) = meta.lit {
                    let ttt = s
                        .parse_with(syn::Path::parse_mod_style)
                        .expect("Must be types for deps");

                    let field = syn::Field {
                        attrs: Vec::new(),
                        vis: parse_quote!(pub),
                        ident: name.cloned(),
                        colon_token: Some(Default::default()),
                        ty: parse_quote!(abcf::manager::context::RDependence<'__abcf_deps, #ttt<S, D>>),
                    };

                    r_fields.push(field);

                    let field = syn::Field {
                        attrs: Vec::new(),
                        vis: parse_quote!(pub),
                        ident: name.cloned(),
                        colon_token: Some(Default::default()),
                        ty: parse_quote!(abcf::manager::context::ADependence<'__abcf_deps, #ttt<S, D>>),
                    };

                    a_fields.push(field);

                    let field = syn::Field {
                        attrs: Vec::new(),
                        vis: parse_quote!(pub),
                        ident: name.cloned(),
                        colon_token: Some(Default::default()),
                        ty: parse_quote!(abcf::manager::context::TDependence<'__abcf_deps, #ttt<S, D>>),
                    };

                    t_fields.push(field);
                }
            }

            let r_struct: ItemStruct = parse_quote!(
                pub struct #rpc_ident<
                    '__abcf_deps,
                    S: abcf::bs3::Store + 'static,
                    D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send + core::clone::Clone
                > {
                    #(#r_fields,)*
                    pub __marker_s: core::marker::PhantomData<&'__abcf_deps S>,
                    pub __marker_d: core::marker::PhantomData<&'__abcf_deps D>,
                }
            );

            let a_struct: ItemStruct = parse_quote!(
                pub struct #app_ident<
                    '__abcf_deps,
                    S: abcf::bs3::Store + 'static,
                    D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send + core::clone::Clone
                > {
                    #(#a_fields,)*
                    pub __marker_s: core::marker::PhantomData<&'__abcf_deps S>,
                    pub __marker_d: core::marker::PhantomData<&'__abcf_deps D>,
                }
            );

            let t_struct: ItemStruct = parse_quote!(
                pub struct #txn_ident<
                    '__abcf_deps,
                    S: abcf::bs3::Store + 'static,
                    D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send + core::clone::Clone
                > {
                    #(#t_fields,)*
                    pub __marker_s: core::marker::PhantomData<&'__abcf_deps S>,
                    pub __marker_d: core::marker::PhantomData<&'__abcf_deps D>,
                }
            );

            result_item.push(r_struct);
            result_item.push(a_struct);
            result_item.push(t_struct);
        }
    }

    if result_item.len() == 0 {
        let r_struct: ItemStruct = parse_quote!(
            pub struct #rpc_ident<
                '__abcf_deps,
                S: abcf::bs3::Store + 'static,
                D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send + core::clone::Clone
            > {
                pub __marker_s: core::marker::PhantomData<&'__abcf_deps S>,
                pub __marker_d: core::marker::PhantomData<&'__abcf_deps D>,
            }
        );

        let a_struct: ItemStruct = parse_quote!(
            pub struct #app_ident<
                '__abcf_deps,
                S: abcf::bs3::Store + 'static,
                D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send + core::clone::Clone
            > {
                pub __marker_s: core::marker::PhantomData<&'__abcf_deps S>,
                pub __marker_d: core::marker::PhantomData<&'__abcf_deps D>,
            }
        );

        let t_struct: ItemStruct = parse_quote!(
            pub struct #txn_ident<
                '__abcf_deps,
                S: abcf::bs3::Store + 'static,
                D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send + core::clone::Clone
            > {
                pub __marker_s: core::marker::PhantomData<&'__abcf_deps S>,
                pub __marker_d: core::marker::PhantomData<&'__abcf_deps D>,
            }
        );

        result_item.push(r_struct);
        result_item.push(a_struct);
        result_item.push(t_struct);
    }

    let mut deps_trait: ItemImpl = parse_quote! {
        impl abcf::manager::Dependence for #store_name<#(#lifetime_names,)* #(#generics_names,)*> {
            type RPC<'__abcf_deps> = #namespace_ident::#rpc_ident<'__abcf_deps, #(#lifetime_names,)* #(#generics_names,)*>;
            type App<'__abcf_deps> = #namespace_ident::#app_ident<'__abcf_deps, #(#lifetime_names,)* #(#generics_names,)*>;
            type Txn<'__abcf_deps> = #namespace_ident::#txn_ident<'__abcf_deps, #(#lifetime_names,)* #(#generics_names,)*>;
        }
    };

    deps_trait.generics = generics.clone();

    (result_item, deps_trait)
}

/// Define Module
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as PunctuatedMetaNameValue);
    let mut parsed = parse_macro_input!(input as ItemStruct);

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

    let mut init_items = Vec::new();
    let mut fn_items = Vec::new();

    let mut merkle_items = Vec::new();

    let mut stateless_tree_match_arms = Vec::new();
    let mut stateful_tree_match_arms = Vec::new();

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
                    target_field.ty = parse_quote!(abcf::bs3::SnapshotableStorage<S, abcf::bs3::merkle::empty::EmptyMerkle<D>, #origin_ty>);
                    stateless.push(target_field.clone());

                    target_field.ty = parse_quote!(abcf::bs3::Transaction<'a, S,  abcf::bs3::merkle::empty::EmptyMerkle<D>, #origin_ty>);
                    stateless_tx.push(target_field.clone());

                    is_memory = false;

                    let target_field_ident = target_field.ident.clone().unwrap();
                    let target_field_ident_str = target_field.ident.clone().unwrap().to_string();
                    let stateless_tree_arm: Arm = parse_quote!(#target_field_ident_str => {
                            let mut ss = self.#target_field_ident.clone();

                            if height > 0 {
                                ss.rollback(height)
                                    .map_err(|_|abcf::ModuleError {
                                        namespace: String::from(#name),
                                        error: abcf::Error::QueryPathFormatError,
                                    })?;
                            }

                            let v = ss.tree_get(&key_vec.to_vec())
                                .map_err(|_|abcf::ModuleError {
                                    namespace: String::from(#name),
                                    error: abcf::Error::QueryPathFormatError,
                                })?;
                            Ok(v)
                        }
                    );

                    stateless_tree_match_arms.push(stateless_tree_arm);
                } else if attr.path.is_ident("stateful") {
                    let parsed: FieldParsedMetaName = attr.parse_args().expect("parsed error");

                    let merkle = parsed
                        .merkle
                        .parse_with(syn::Path::parse_mod_style)
                        .expect("must be path");

                    let mut target_field = f.clone();

                    stateful_value.push(target_field.clone());

                    let origin_ty = f.ty.clone();
                    stateful_arg.push(target_field.ident.clone().unwrap());
                    target_field.ty =
                        parse_quote!(abcf::bs3::SnapshotableStorage<S, #merkle<D>, #origin_ty>);
                    stateful.push(target_field.clone());

                    target_field.ty =
                        parse_quote!(abcf::bs3::Transaction<'a, S, #merkle<D>, #origin_ty>);
                    stateful_tx.push(target_field.clone());

                    let stateful_name = f.ident.clone().unwrap();

                    let merkle_stmt: syn::Expr = parse_quote!(
                        self.#stateful_name.root()?
                    );
                    merkle_items.push(merkle_stmt);

                    is_memory = false;

                    let target_field_ident = target_field.ident.clone().unwrap();
                    let target_field_ident_str = target_field.ident.clone().unwrap().to_string();
                    let stateful_tree_arm: Arm = parse_quote!(#target_field_ident_str => {
                            let mut ss = self.#target_field_ident.clone();

                            if height > 0 {
                                ss.rollback(height)
                                    .map_err(|_|abcf::ModuleError {
                                        namespace: String::from(#name),
                                        error: abcf::Error::QueryPathFormatError,
                                    })?;
                            }

                            let v = ss.tree_get(&key_vec.to_vec())
                                .map_err(|_|abcf::ModuleError {
                                    namespace: String::from(#name),
                                    error: abcf::Error::QueryPathFormatError,
                                })?;
                            Ok(v)
                        }
                    );

                    stateful_tree_match_arms.push(stateful_tree_arm);
                }
            }
            if is_memory {
                fields.named.push(f.clone());
                let key = f.ident.clone().expect("module muse a named struct");
                let ty = f.ty.clone();

                let fv: FieldValue = parse_quote!(#key);
                let fa: FnArg = parse_quote!(#key: #ty);
                init_items.push(fv);
                fn_items.push(fa);
            }
        }
    }

    let stateless_struct_ident =
        Ident::new(&format!("ABCFModule{}Sl", parsed.ident), Span::call_site());

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

    parsed
        .generics
        .params
        .push(parse_quote!(S: abcf::bs3::Store + 'static));

    parsed.generics.params.push(
        parse_quote!(D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send + core::clone::Clone),
    );

    let mut generics_names = Vec::new();
    let mut lifetime_names = Vec::new();
    let mut markers = Vec::new();
    let mut markers_with_generics = Vec::new();

    for x in &parsed.generics.params {
        if let GenericParam::Type(t) = x {
            generics_names.push(t.ident.clone());
            let g = t.ident.clone();

            let marker_name_str =
                format!("__marker_{}", t.ident.clone().to_string().to_lowercase());
            let marker_key = Ident::new(marker_name_str.as_str(), Span::call_site());

            let fields: FieldValue = parse_quote! (#marker_key: core::marker::PhantomData);
            let fields_with_g: ParseField =
                parse_quote! (pub #marker_key: core::marker::PhantomData<#g>);

            markers.push(fields);
            markers_with_generics.push(fields_with_g);
        } else if let GenericParam::Lifetime(l) = x {
            lifetime_names.push(l.lifetime.clone());
        }
    }

    let attrs = std::mem::take(&mut parsed.attrs);

    let (dep_items, dep_impl) = build_dependence_for_module(
        &parsed.ident,
        &storage_module_ident,
        &parsed.generics,
        &attrs,
        &generics_names,
        &lifetime_names,
    );

    if let Fields::Named(fields) = &mut parsed.fields {
        for x in markers_with_generics.clone() {
            fields.named.push(x.inner);
        }
    };

    let mut new_impl: ItemImpl = parse_quote! {
        impl #module_name<#(#lifetime_names,)* #(#generics_names,)*> {
            pub fn new(#(#fn_items,)*) -> Self {
                Self {
                    #(#init_items,)*
                    #(#markers,)*
                }
            }
        }
    };

    new_impl.generics = parsed.generics.clone();

    let mut store_trait: ItemImpl = parse_quote! {
        impl abcf::manager::ModuleStorage for #module_name<#(#lifetime_names,)* #(#generics_names,)*> {
            type Stateless = #storage_module_ident::#stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>;
            type Stateful = #storage_module_ident::#stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>;
        }
    };

    store_trait.generics = parsed.generics.clone();

    let mut metadata_trait: ItemImpl = parse_quote! {
        impl abcf::Module for #module_name<#(#lifetime_names,)* #(#generics_names,)*> {
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
    };

    metadata_trait.generics = parsed.generics.clone();

    let __module1_sf = stateful_arg.first().unwrap();
    let __module1_sl = stateless_arg.first().unwrap();

    let mut stateless_struct: ItemStruct = parse_quote! {
        pub struct #stateless_struct_ident
            {
                #(#stateless,)*
                #(#markers_with_generics,)*
            }
    };

    stateless_struct.generics = parsed.generics.clone();

    let mut stateless_struct_tree: ItemImpl = parse_quote! {
        impl abcf::entry::Tree for #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn get(&mut self, key: &str, height: i64) -> abcf::ModuleResult<Vec<u8>> {
                use abcf::bs3::prelude::Tree;

                let mut splited = key.splitn(2, "/");

                let store_name = splited.next().ok_or(abcf::ModuleError {
                    namespace: String::from(#name),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                let inner_key = splited.next().ok_or(abcf::ModuleError {
                    namespace: String::from(#name),
                    error: abcf::Error::QueryPathFormatError,
                })?;


                let inner_key = &inner_key.to_string()[2..];


                let mut key_vec = abcf::hex::decode(inner_key)
                    .map_err(|e|{
                    abcf::log::debug!("hex::decode:{}",e.to_string());
                    abcf::ModuleError::new(#name,
                        abcf::Error::QueryPathFormatError)
                })?;

                match store_name {
                    #(#stateless_tree_match_arms,)*
                    _ => Err(abcf::ModuleError {
                        namespace: String::from(#name),
                        error: abcf::Error::NoModule,
                    })
                }
            }
        }
    };
    stateless_struct_tree.generics = parsed.generics.clone();

    let mut sl_tx: ItemStruct = parse_quote! {
        pub struct #stateless_tx_struct_ident {
            #(#stateless_tx,)*
            #(#markers_with_generics,)*
        }
    };

    sl_tx.generics = parsed.generics.clone();
    sl_tx.generics.params.push(parse_quote!('a));

    let mut sl_tx_methods: ItemImpl = parse_quote! {
        impl #stateless_tx_struct_ident<'a, #(#lifetime_names,)* #(#generics_names,)*> {
            pub fn execute(&mut self, transaction: #stateless_tx_cache_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>) {
                #(self.#stateless_arg.execute(transaction.#stateless_arg);)*
            }
        }
    };

    sl_tx_methods.generics = parsed.generics.clone();
    sl_tx_methods.generics.params.push(parse_quote!('a));

    let mut sl_tx_clone: ItemImpl = parse_quote! {
        impl Clone for #stateless_tx_struct_ident<'a, #(#lifetime_names,)* #(#generics_names,)*> {
            fn clone(&self) -> Self {
                Self {
                    #(#stateless_arg: self.#stateless_arg.clone(),)*
                    #(#markers,)*
                }
            }
        }
    };

    sl_tx_clone.generics = parsed.generics.clone();
    sl_tx_clone.generics.params.push(parse_quote!('a));

    let mut sl_cache: ItemStruct = parse_quote! {
        pub struct #stateless_tx_cache_struct_ident {
            #(#stateless_value,)*
            #(#markers_with_generics,)*
        }
    };
    sl_cache.generics = parsed.generics.clone();

    let mut sl_storage_impl: ItemImpl = parse_quote! {
        impl abcf::Storage for #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn rollback(&mut self, height: i64) -> Result<()> {
                #(
                    self.#stateless_arg.rollback(height)?;
                )*
                Ok(())
            }

            fn height(&self) -> Result<i64> {
                Ok(self.#__module1_sl.height)
            }

            fn commit(&mut self) -> Result<()> {
                #(
                    self.#stateless_arg.commit()?;
                )*
                Ok(())
            }
        }
    };
    sl_storage_impl.generics = parsed.generics.clone();

    let mut sl_storage_tx_impl: ItemImpl = parse_quote! {
        impl StorageTransaction for #stateless_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            type Transaction<'a> = #stateless_tx_struct_ident<'a, #(#lifetime_names,)* #(#generics_names,)*>;

            type Cache = #stateless_tx_cache_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>;

            fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
                Self::Cache {
                    #(#stateless_arg: tx.#stateless_arg.value,)*
                    #(#markers,)*
                }
            }

            fn transaction(&self) -> Self::Transaction<'_> {
                #stateless_tx_struct_ident {
                    #(#stateless_arg: self.#stateless_arg.transaction(),)*
                    #(#markers,)*
                }
            }

            fn execute(&mut self, transaction: Self::Cache) {
                #(self.#stateless_arg.execute(transaction.#stateless_arg);)*
            }
        }
    };
    sl_storage_tx_impl.generics = parsed.generics.clone();

    let mut stateful_struct: ItemStruct = parse_quote! {
        pub struct #stateful_struct_ident {
            #(#stateful,)*
            #(#markers_with_generics,)*
        }
    };
    stateful_struct.generics = parsed.generics.clone();

    let mut sf_tx: ItemStruct = parse_quote! {
        pub struct #stateful_tx_struct_ident {
            #(#stateful_tx,)*
            #(#markers_with_generics,)*
        }
    };

    sf_tx.generics = parsed.generics.clone();
    sf_tx.generics.params.push(parse_quote!('a));

    let mut sf_tx_methods: ItemImpl = parse_quote! {
        impl #stateful_tx_struct_ident<'a, #(#lifetime_names,)* #(#generics_names,)*> {
            pub fn execute(&mut self, transaction: #stateful_tx_cache_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>) {
                #(self.#stateful_arg.execute(transaction.#stateful_arg);)*
            }
        }
    };

    sf_tx_methods.generics = parsed.generics.clone();
    sf_tx_methods.generics.params.push(parse_quote!('a));

    let mut sf_tx_clone: ItemImpl = parse_quote! {
        impl Clone for #stateful_tx_struct_ident<'a, #(#lifetime_names,)* #(#generics_names,)*> {
            fn clone(&self) -> Self {
                Self {
                    #(#stateful_arg: self.#stateful_arg.clone(),)*
                    #(#markers,)*
                }
            }
        }
    };

    sf_tx_clone.generics = parsed.generics.clone();
    sf_tx_clone.generics.params.push(parse_quote!('a));

    let mut sf_cache: ItemStruct = parse_quote! {
        pub struct #stateful_tx_cache_struct_ident {
            #(#stateful_value,)*
            #(#markers_with_generics,)*
        }
    };
    sf_cache.generics = parsed.generics.clone();

    let mut sf_storage_impl: ItemImpl = parse_quote! {
        impl abcf::Storage for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn rollback(&mut self, height: i64) -> Result<()> {
                #(
                    self.#stateful_arg.rollback(height)?;
                )*
                Ok(())
            }

            fn height(&self) -> Result<i64> {
                Ok(self.#__module1_sf.height)
            }

            fn commit(&mut self) -> Result<()> {
                #(
                    self.#stateful_arg.commit()?;
                )*
                Ok(())
            }
        }
    };
    sf_storage_impl.generics = parsed.generics.clone();

    let mut sf_storage_tx_impl: ItemImpl = parse_quote! {
        impl abcf::module::StorageTransaction for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            type Transaction<'a> = #stateful_tx_struct_ident<'a, #(#lifetime_names,)* #(#generics_names,)*>;

            type Cache = #stateful_tx_cache_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>;

            fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
                Self::Cache {
                    #(#stateful_arg: tx.#stateful_arg.value,)*
                    #(#markers,)*
                }
            }

            fn transaction(&self) -> Self::Transaction<'_> {
                Self::Transaction::<'_> {
                    #(#stateful_arg: self.#stateful_arg.transaction(),)*
                    #(#markers,)*
                }
            }

            fn execute(&mut self, transaction: Self::Cache) {
                #(self.#stateful_arg.execute(transaction.#stateful_arg);)*
            }
        }
    };
    sf_storage_tx_impl.generics = parsed.generics.clone();

    let stateful_merkle: ItemImpl = parse_quote! {
        impl<S, D> abcf::module::Merkle<D> for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*>
        where
            S: abcf::bs3::Store,
            D: abcf::digest::Digest + core::marker::Sync + core::marker::Send + core::clone::Clone,
        {
            fn root(&self) -> Result<abcf::digest::Output<D>> {
                let mut hasher = D::new();

                #(hasher.update(#merkle_items);)*

                Ok(hasher.finalize())
            }
        }
    };

    let mut stateful_struct_tree: ItemImpl = parse_quote! {
        impl abcf::entry::Tree for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn get(&mut self, key: &str, height: i64) -> abcf::ModuleResult<Vec<u8>> {
                use abcf::bs3::prelude::Tree;

                let mut splited = key.splitn(2, "/");

                let store_name = splited.next().ok_or(abcf::ModuleError {
                    namespace: String::from(#name),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                let inner_key = splited.next().ok_or(abcf::ModuleError {
                    namespace: String::from(#name),
                    error: abcf::Error::QueryPathFormatError,
                })?;

                let mut key_vec = abcf::hex::decode(inner_key)
                    .map_err(|e|{
                    abcf::log::debug!("hex::decode:{}",e.to_string());
                    abcf::ModuleError::new(#name,
                        abcf::Error::QueryPathFormatError)
                })?;

                match store_name {
                    #(#stateful_tree_match_arms,)*
                    _ => Err(abcf::ModuleError {
                        namespace: String::from(#name),
                        error: abcf::Error::NoModule,
                    })
                }
            }
        }
    };
    stateful_struct_tree.generics = parsed.generics.clone();

    let result = quote! {
        #parsed

        #new_impl

        #store_trait

        #metadata_trait

        #dep_impl

        pub mod #storage_module_ident {
            use super::*;
            use abcf::Result;
            use abcf::module::StorageTransaction;

            pub const MODULE_NAME: &'static str = #name;

            #(#dep_items)*

            #stateless_struct

            #stateless_struct_tree

            #sl_tx

            #sl_tx_methods

            #sl_tx_clone

            #sl_cache

            #sl_storage_impl

            #sl_storage_tx_impl

            #stateful_struct

            #sf_tx

            #sf_tx_methods

            #sf_tx_clone

            #sf_cache

            #sf_storage_impl

            #sf_storage_tx_impl

            #stateful_merkle

            #stateful_struct_tree

        }
    };

    TokenStream::from(result)
}
