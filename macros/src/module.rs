use crate::utils::ParseField;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::mem::replace;
use syn::{
    parse::Parse, parse_macro_input, parse_quote, punctuated::Punctuated, Attribute, FieldValue,
    Fields, FnArg, GenericParam, ItemImpl, ItemStruct, Lit, LitStr, MetaNameValue, Token,
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

pub fn build_dependence_for_module(store_name: &Ident, attrs: &[Attribute]) -> Option<ItemStruct> {
    for attr in attrs {
        if attr.path.is_ident("dependence") {
            let parser = Punctuated::<MetaNameValue, Token![,]>::parse_terminated;
            let metas = attr.parse_args_with(parser).unwrap();

            let mut v = Vec::new();

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
                        ty: parse_quote!(abcf::manager::Dependence<'a, #ttt<S, D>, abcf::Stateless<#ttt<S, D>>, abcf::Stateful<#ttt<S, D>>),
                    };

                    v.push(field);
                }
            }

            let stateful = parse_quote!(
                pub struct #store_name<
                    'a,
                    S: abcf::bs3::Store + 'static,
                    D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send
                > {
                    #(#v,)*
                }
            );

            return Some(stateful);
        }
    }
    None
}

/// Define Module
pub fn module(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as PunctuatedMetaNameValue);
    let mut parsed = parse_macro_input!(input as ItemStruct);

    let name = args.name;
    let version = args.version;
    let impl_version = args.impl_version;
    let target_height = args.target_height;

    let attrs = std::mem::take(&mut parsed.attrs);

    let deps_ident = Ident::new(
        &format!("ABCFDeps{}", parsed.ident.to_string()),
        Span::call_site(),
    );

    let deps = build_dependence_for_module(&deps_ident, &attrs);

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
                    stateless_tx.push(target_field);

                    is_memory = false;
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
                    stateful_tx.push(target_field);

                    is_memory = false;
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

    parsed
        .generics
        .params
        .push(parse_quote!(S: abcf::bs3::Store + 'static));

    parsed.generics.params.push(
        parse_quote!(D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send),
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

    let impl_deps = if deps.is_some() {
        let mut impl_deps: ItemImpl = parse_quote!(
            impl abcf::manager::ModuleStorageDependence<'__abcf_dep> for #module_name<#(#lifetime_names,)* #(#generics_names,)*> {
                type Dependence = #storage_module_ident::#deps_ident<'__abcf_dep, #(#lifetime_names,)* #(#generics_names,)*>;
            }
        );

        impl_deps.generics = parsed.generics.clone();
        impl_deps
            .generics
            .params
            .insert(0, parse_quote!('__abcf_dep));

        Some(impl_deps)
    } else {
        let mut impl_deps: ItemImpl = parse_quote!(
            impl abcf::manager::ModuleStorageDependence<'__abcf_dep> for #module_name<#(#lifetime_names,)* #(#generics_names,)*> {
                type Dependence = ();
            }
        );

        impl_deps.generics = parsed.generics.clone();
        impl_deps
            .generics
            .params
            .insert(0, parse_quote!('__abcf_dep));

        Some(impl_deps)
    };

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
            fn get(&self, _key: &str, _height: i64) -> abcf::ModuleResult<Vec<u8>> {
                Ok(Vec::new())
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
            D: abcf::digest::Digest + core::marker::Sync + core::marker::Send,
        {
            fn root(&self) -> Result<abcf::digest::Output<D>> {
                Ok(Default::default())
            }
        }
    };

    let mut stateful_struct_tree: ItemImpl = parse_quote! {
        impl abcf::entry::Tree for #stateful_struct_ident<#(#lifetime_names,)* #(#generics_names,)*> {
            fn get(&self, _key: &str, _height: i64) -> abcf::ModuleResult<Vec<u8>> {
                Ok(Vec::new())
            }
        }
    };
    stateful_struct_tree.generics = parsed.generics.clone();

    let result = quote! {
        #parsed

        #new_impl

        #store_trait

        #metadata_trait

        #impl_deps

        pub mod #storage_module_ident {
            use super::*;
            use abcf::Result;
            use abcf::module::StorageTransaction;

            pub const MODULE_NAME: &'static str = #name;

            #deps

            #stateless_struct

            #stateless_struct_tree

            #sl_tx

            #sl_cache

            #sl_storage_impl

            #sl_storage_tx_impl

            #stateful_struct

            #sf_tx

            #sf_cache

            #sf_storage_impl

            #sf_storage_tx_impl

            #stateful_merkle

            #stateful_struct_tree

        }
    };

    TokenStream::from(result)
}
