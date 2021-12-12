#![feature(prelude_import)]
#![feature(generic_associated_types)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{Map, Value},
    },
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};
use serde::{Deserialize, Serialize};
use sha3::Sha3_512;
pub struct MockModule<
    S: abcf::bs3::Store + 'static,
    D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
> {
    pub inner: u32,
    pub __marker_s: core::marker::PhantomData<S>,
    pub __marker_d: core::marker::PhantomData<D>,
}
impl<
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > MockModule<S, D>
{
    pub fn new(inner: u32) -> Self {
        Self {
            inner,
            __marker_s: core::marker::PhantomData,
            __marker_d: core::marker::PhantomData,
        }
    }
}
impl<
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > abcf::manager::ModuleStorage for MockModule<S, D>
{
    type Stateless = __abcf_storage_mockmodule::ABCFModuleMockModuleSl<S, D>;
    type Stateful = __abcf_storage_mockmodule::ABCFModuleMockModuleSf<S, D>;
}
impl<
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > abcf::Module for MockModule<S, D>
{
    fn metadata(&self) -> abcf::ModuleMetadata<'_> {
        abcf::ModuleMetadata {
            name: "mock",
            version: 1,
            impl_version: "0.1.1",
            module_type: abcf::ModuleType::Module,
            genesis: abcf::Genesis { target_height: 0 },
        }
    }
}
impl<
        '__abcf_dep,
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > abcf::manager::ModuleStorageDependence<'__abcf_dep> for MockModule<S, D>
{
    type Dependence = ();
}
pub mod __abcf_storage_mockmodule {
    use super::*;
    use abcf::module::StorageTransaction;
    use abcf::Result;
    pub const MODULE_NAME: &'static str = "mock";
    pub struct ABCFModuleMockModuleSl<
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > {
        pub sl_value:
            abcf::bs3::SnapshotableStorage<S, abcf::bs3::merkle::empty::EmptyMerkle<D>, Value<u32>>,
        pub sl_map: abcf::bs3::SnapshotableStorage<
            S,
            abcf::bs3::merkle::empty::EmptyMerkle<D>,
            Map<i32, u32>,
        >,
        pub __marker_s: core::marker::PhantomData<S>,
        pub __marker_d: core::marker::PhantomData<D>,
    }
    impl<
            S: abcf::bs3::Store + 'static,
            D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
        > abcf::entry::Tree for ABCFModuleMockModuleSl<S, D>
    {
        fn get(&self, _key: &str, _height: i64) -> abcf::ModuleResult<Vec<u8>> {
            Ok(Vec::new())
        }
    }
    pub struct ABCFModuleMockModuleSlTx<
        'a,
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > {
        pub sl_value:
            abcf::bs3::Transaction<'a, S, abcf::bs3::merkle::empty::EmptyMerkle<D>, Value<u32>>,
        pub sl_map:
            abcf::bs3::Transaction<'a, S, abcf::bs3::merkle::empty::EmptyMerkle<D>, Map<i32, u32>>,
        pub __marker_s: core::marker::PhantomData<S>,
        pub __marker_d: core::marker::PhantomData<D>,
    }
    pub struct ABCFModuleMockModuleSlTxCache<
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > {
        pub sl_value: Value<u32>,
        pub sl_map: Map<i32, u32>,
        pub __marker_s: core::marker::PhantomData<S>,
        pub __marker_d: core::marker::PhantomData<D>,
    }
    impl<
            S: abcf::bs3::Store + 'static,
            D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
        > abcf::Storage for ABCFModuleMockModuleSl<S, D>
    {
        fn rollback(&mut self, height: i64) -> Result<()> {
            self.sl_value.rollback(height)?;
            self.sl_map.rollback(height)?;
            Ok(())
        }
        fn height(&self) -> Result<i64> {
            Ok(self.sl_value.height)
        }
        fn commit(&mut self) -> Result<()> {
            self.sl_value.commit()?;
            self.sl_map.commit()?;
            Ok(())
        }
    }
    impl<
            S: abcf::bs3::Store + 'static,
            D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
        > StorageTransaction for ABCFModuleMockModuleSl<S, D>
    {
        type Transaction<'a> = ABCFModuleMockModuleSlTx<'a, S, D>;
        type Cache = ABCFModuleMockModuleSlTxCache<S, D>;
        fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
            Self::Cache {
                sl_value: tx.sl_value.value,
                sl_map: tx.sl_map.value,
                __marker_s: core::marker::PhantomData,
                __marker_d: core::marker::PhantomData,
            }
        }
        fn transaction(&self) -> Self::Transaction<'_> {
            ABCFModuleMockModuleSlTx {
                sl_value: self.sl_value.transaction(),
                sl_map: self.sl_map.transaction(),
                __marker_s: core::marker::PhantomData,
                __marker_d: core::marker::PhantomData,
            }
        }
        fn execute(&mut self, transaction: Self::Cache) {
            self.sl_value.execute(transaction.sl_value);
            self.sl_map.execute(transaction.sl_map);
        }
    }
    pub struct ABCFModuleMockModuleSf<
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > {
        pub sf_value: abcf::bs3::SnapshotableStorage<S, AppendOnlyMerkle<D>, Value<u32>>,
        pub __marker_s: core::marker::PhantomData<S>,
        pub __marker_d: core::marker::PhantomData<D>,
    }
    pub struct ABCFModuleMockModuleSfTx<
        'a,
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > {
        pub sf_value: abcf::bs3::Transaction<'a, S, AppendOnlyMerkle<D>, Value<u32>>,
        pub __marker_s: core::marker::PhantomData<S>,
        pub __marker_d: core::marker::PhantomData<D>,
    }
    pub struct ABCFModuleMockModuleSfTxCache<
        S: abcf::bs3::Store + 'static,
        D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
    > {
        pub sf_value: Value<u32>,
        pub __marker_s: core::marker::PhantomData<S>,
        pub __marker_d: core::marker::PhantomData<D>,
    }
    impl<
            S: abcf::bs3::Store + 'static,
            D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
        > abcf::Storage for ABCFModuleMockModuleSf<S, D>
    {
        fn rollback(&mut self, height: i64) -> Result<()> {
            self.sf_value.rollback(height)?;
            Ok(())
        }
        fn height(&self) -> Result<i64> {
            Ok(self.sf_value.height)
        }
        fn commit(&mut self) -> Result<()> {
            self.sf_value.commit()?;
            Ok(())
        }
    }
    impl<
            S: abcf::bs3::Store + 'static,
            D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
        > abcf::module::StorageTransaction for ABCFModuleMockModuleSf<S, D>
    {
        type Transaction<'a> = ABCFModuleMockModuleSfTx<'a, S, D>;
        type Cache = ABCFModuleMockModuleSfTxCache<S, D>;
        fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
            Self::Cache {
                sf_value: tx.sf_value.value,
                __marker_s: core::marker::PhantomData,
                __marker_d: core::marker::PhantomData,
            }
        }
        fn transaction(&self) -> Self::Transaction<'_> {
            Self::Transaction::<'_> {
                sf_value: self.sf_value.transaction(),
                __marker_s: core::marker::PhantomData,
                __marker_d: core::marker::PhantomData,
            }
        }
        fn execute(&mut self, transaction: Self::Cache) {
            self.sf_value.execute(transaction.sf_value);
        }
    }
    impl<S, D> abcf::module::Merkle<D> for ABCFModuleMockModuleSf<S, D>
    where
        S: abcf::bs3::Store,
        D: abcf::digest::Digest + core::marker::Sync + core::marker::Send,
    {
        fn root(&self) -> Result<abcf::digest::Output<D>> {
            Ok(Default::default())
        }
    }
    impl<
            S: abcf::bs3::Store + 'static,
            D: abcf::digest::Digest + 'static + core::marker::Sync + core::marker::Send,
        > abcf::entry::Tree for ABCFModuleMockModuleSf<S, D>
    {
        fn get(&self, _key: &str, _height: i64) -> abcf::ModuleResult<Vec<u8>> {
            Ok(Vec::new())
        }
    }
}
impl<S: abcf::bs3::Store, D: abcf::digest::Digest + core::marker::Sync + core::marker::Send>
    MockModule<S, D>
{
}
impl<S: abcf::bs3::Store, D: abcf::digest::Digest + core::marker::Sync + core::marker::Send>
    abcf::RPCs for MockModule<S, D>
{
    #[allow(
        clippy::let_unit_value,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn call<'a, 'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        ctx: abcf::RPCContext<'a, Self>,
        method: &'life1 str,
        params: serde_json::Value,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = abcf::Result<Option<serde_json::Value>>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'a: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<abcf::Result<Option<serde_json::Value>>>
            {
                return __ret;
            }
            let mut __self = self;
            let ctx = ctx;
            let method = method;
            let params = params;
            let __ret: abcf::Result<Option<serde_json::Value>> = { Ok(None) };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
impl<S: abcf::bs3::Store, D: abcf::digest::Digest + core::marker::Sync + core::marker::Send>
    Application for MockModule<S, D>
{
    type Transaction = MockTransaction;
    #[allow(
        clippy::let_unit_value,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn check_tx<'a, 'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        _context: TxnContext<'a, Self>,
        _req: &'life1 RequestCheckTx<Self::Transaction>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = abcf::Result<ResponseCheckTx>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'a: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<abcf::Result<ResponseCheckTx>>
            {
                return __ret;
            }
            let mut __self = self;
            let _context = _context;
            let _req = _req;
            let __ret: abcf::Result<ResponseCheckTx> = { Ok(Default::default()) };
            #[allow(unreachable_code)]
            __ret
        })
    }
    #[allow(
        clippy::let_unit_value,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds,
        clippy::used_underscore_binding
    )]
    fn deliver_tx<'a, 'life0, 'life1, 'async_trait>(
        &'life0 mut self,
        _context: TxnContext<'a, Self>,
        _req: &'life1 RequestDeliverTx<Self::Transaction>,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = abcf::Result<ResponseDeliverTx>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'a: 'async_trait,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
    {
        Box::pin(async move {
            if let ::core::option::Option::Some(__ret) =
                ::core::option::Option::None::<abcf::Result<ResponseDeliverTx>>
            {
                return __ret;
            }
            let mut __self = self;
            let _context = _context;
            let _req = _req;
            let __ret: abcf::Result<ResponseDeliverTx> = { Ok(Default::default()) };
            #[allow(unreachable_code)]
            __ret
        })
    }
}
pub struct MockTransaction {}
impl Default for MockTransaction {
    fn default() -> Self {
        MockTransaction {}
    }
}
pub struct SimpleNodeTransaction {
    pub v: u64,
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for SimpleNodeTransaction {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = match _serde::Serializer::serialize_struct(
                __serializer,
                "SimpleNodeTransaction",
                false as usize + 1,
            ) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            match _serde::ser::SerializeStruct::serialize_field(&mut __serde_state, "v", &self.v) {
                _serde::__private::Ok(__val) => __val,
                _serde::__private::Err(__err) => {
                    return _serde::__private::Err(__err);
                }
            };
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for SimpleNodeTransaction {
        fn deserialize<__D>(__deserializer: __D) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __ignore,
            }
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(__formatter, "field identifier")
                }
                fn visit_u64<__E>(self, __value: u64) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "v" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"v" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(__deserializer, __FieldVisitor)
                }
            }
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<SimpleNodeTransaction>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = SimpleNodeTransaction;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct SimpleNodeTransaction",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 =
                        match match _serde::de::SeqAccess::next_element::<u64>(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(_serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct SimpleNodeTransaction with 1 element",
                                ));
                            }
                        };
                    _serde::__private::Ok(SimpleNodeTransaction { v: __field0 })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<u64> = _serde::__private::None;
                    while let _serde::__private::Some(__key) =
                        match _serde::de::MapAccess::next_key::<__Field>(&mut __map) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        }
                    {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("v"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    match _serde::de::MapAccess::next_value::<u64>(&mut __map) {
                                        _serde::__private::Ok(__val) => __val,
                                        _serde::__private::Err(__err) => {
                                            return _serde::__private::Err(__err);
                                        }
                                    },
                                );
                            }
                            _ => {
                                let _ = match _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)
                                {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                };
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            match _serde::__private::de::missing_field("v") {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            }
                        }
                    };
                    _serde::__private::Ok(SimpleNodeTransaction { v: __field0 })
                }
            }
            const FIELDS: &'static [&'static str] = &["v"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "SimpleNodeTransaction",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<SimpleNodeTransaction>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
impl abcf::Transaction for SimpleNodeTransaction {}
impl Default for SimpleNodeTransaction {
    fn default() -> Self {
        Self { v: 0 }
    }
}
impl abcf::module::FromBytes for SimpleNodeTransaction {
    fn from_bytes(bytes: &[u8]) -> abcf::Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_slice(bytes)?)
    }
}
impl TryFrom<&SimpleNodeTransaction> for MockTransaction {
    type Error = abcf::Error;
    fn try_from(_: &SimpleNodeTransaction) -> Result<Self, Self::Error> {
        Ok(MockTransaction {})
    }
}
pub struct SimpleManager<S: abcf::bs3::Store + 'static> {
    pub mock: MockModule<S, Sha3_512>,
    pub mock2: MockModule<S, Sha3_512>,
    __marker_s: core::marker::PhantomData<S>,
    __calls: abcf::manager::CallImpl,
}
impl<S: abcf::bs3::Store + 'static> SimpleManager<S> {
    pub fn new(mock: MockModule<S, Sha3_512>, mock2: MockModule<S, Sha3_512>) -> Self {
        Self {
            mock,
            mock2,
            __marker_s: core::marker::PhantomData,
            __calls: abcf::manager::CallImpl::new(),
        }
    }
}
impl<S: abcf::bs3::Store + 'static> abcf::Module for SimpleManager<S> {
    fn metadata(&self) -> abcf::ModuleMetadata<'_> {
        abcf::ModuleMetadata {
            name: "simple_node",
            version: 0,
            impl_version: "0.1.0",
            module_type: abcf::ModuleType::Manager,
            genesis: abcf::Genesis { target_height: 0 },
        }
    }
}
impl<S: abcf::bs3::Store + 'static> abcf::manager::ModuleStorage for SimpleManager<S> {
    type Stateless = __abcf_storage_simplemanager::ABCFManagerSimpleManagerSl<S>;
    type Stateful = __abcf_storage_simplemanager::ABCFManagerSimpleManagerSf<S>;
}
pub mod __abcf_storage_simplemanager {
    use super::*;
    use abcf::Result;
    pub const MODULE_NAME: &'static str = "simple_node";
    pub struct ABCFManagerSimpleManagerSl<S: abcf::bs3::Store + 'static> {
        pub mock: abcf::Stateless<MockModule<S, Sha3_512>>,
        pub mock2: abcf::Stateless<MockModule<S, Sha3_512>>,
    }
    impl<S: abcf::bs3::Store + 'static> abcf::entry::Tree for ABCFManagerSimpleManagerSl<S> {
        fn get(&self, key: &str, height: i64) -> abcf::ModuleResult<Vec<u8>> {
            let mut splited = key.splitn(2, "/");
            let module_name = splited.next().ok_or(abcf::ModuleError {
                namespace: String::from("simple_node"),
                error: abcf::Error::QueryPathFormatError,
            })?;
            let inner_key = splited.next().ok_or(abcf::ModuleError {
                namespace: String::from("simple_node"),
                error: abcf::Error::QueryPathFormatError,
            })?;
            match module_name {
                "mock" => Ok(self.mock.get(key, height)?),
                "mock2" => Ok(self.mock2.get(key, height)?),
                _ => Err(abcf::ModuleError {
                    namespace: String::from("simple_node"),
                    error: abcf::Error::NoModule,
                }),
            }
        }
    }
    pub struct ABCFManagerSimpleManagerSlTx<'a, S: abcf::bs3::Store + 'static> {
        mock: abcf::StatelessBatch<'a, MockModule<S, Sha3_512>>,
        mock2: abcf::StatelessBatch<'a, MockModule<S, Sha3_512>>,
    }
    pub struct ABCFManagerSimpleManagerSlTxCache<S: abcf::bs3::Store + 'static> {
        mock: abcf::StatelessCache<MockModule<S, Sha3_512>>,
        mock2: abcf::StatelessCache<MockModule<S, Sha3_512>>,
    }
    impl<S: abcf::bs3::Store + 'static> abcf::Storage for ABCFManagerSimpleManagerSl<S> {
        fn rollback(&mut self, height: i64) -> Result<()> {
            self.mock.rollback(height)?;
            self.mock2.rollback(height)?;
            Ok(())
        }
        fn height(&self) -> Result<i64> {
            Ok(self.mock.height()?)
        }
        fn commit(&mut self) -> Result<()> {
            self.mock.commit()?;
            self.mock2.commit()?;
            Ok(())
        }
    }
    impl<S: abcf::bs3::Store + 'static> abcf::module::StorageTransaction
        for ABCFManagerSimpleManagerSl<S>
    {
        type Transaction<'a> = ABCFManagerSimpleManagerSlTx<'a, S>;
        type Cache = ABCFManagerSimpleManagerSlTxCache<S>;
        fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
            Self::Cache {
                mock: abcf::Stateless::<MockModule<S, Sha3_512>>::cache(tx.mock),
                mock2: abcf::Stateless::<MockModule<S, Sha3_512>>::cache(tx.mock2),
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
            self.mock2.execute(transaction.mock2);
        }
    }
    pub struct ABCFManagerSimpleManagerSf<S: abcf::bs3::Store + 'static> {
        pub mock: abcf::Stateful<MockModule<S, Sha3_512>>,
        pub mock2: abcf::Stateful<MockModule<S, Sha3_512>>,
    }
    impl<S: abcf::bs3::Store + 'static> abcf::entry::Tree for ABCFManagerSimpleManagerSf<S> {
        fn get(&self, key: &str, height: i64) -> abcf::ModuleResult<Vec<u8>> {
            let mut splited = key.splitn(2, "/");
            let module_name = splited.next().ok_or(abcf::ModuleError {
                namespace: String::from("simple_node"),
                error: abcf::Error::QueryPathFormatError,
            })?;
            let inner_key = splited.next().ok_or(abcf::ModuleError {
                namespace: String::from("simple_node"),
                error: abcf::Error::QueryPathFormatError,
            })?;
            match module_name {
                "mock" => Ok(self.mock.get(key, height)?),
                "mock2" => Ok(self.mock2.get(key, height)?),
                _ => Err(abcf::ModuleError {
                    namespace: String::from("simple_node"),
                    error: abcf::Error::NoModule,
                }),
            }
        }
    }
    impl<S: abcf::bs3::Store + 'static> abcf::module::Merkle<Sha3_512>
        for ABCFManagerSimpleManagerSf<S>
    {
        fn root(&self) -> abcf::Result<digest::Output<Sha3_512>> {
            use abcf::module::Merkle;
            use digest::Digest;
            let mut hasher = Sha3_512::new();
            {
                let root = self.mock.root()?;
                if root != abcf::digest::Output::<Sha3_512>::default() {
                    hasher.update(root);
                }
            }
            {
                let root = self.mock2.root()?;
                if root != abcf::digest::Output::<Sha3_512>::default() {
                    hasher.update(root);
                }
            }
            Ok(hasher.finalize())
        }
    }
    pub struct ABCFManagerSimpleManagerSfTx<'a, S: abcf::bs3::Store + 'static> {
        mock: abcf::StatefulBatch<'a, MockModule<S, Sha3_512>>,
        mock2: abcf::StatefulBatch<'a, MockModule<S, Sha3_512>>,
    }
    pub struct ABCFManagerSimpleManagerSfTxCache<S: abcf::bs3::Store + 'static> {
        mock: abcf::StatefulCache<MockModule<S, Sha3_512>>,
        mock2: abcf::StatefulCache<MockModule<S, Sha3_512>>,
    }
    impl<S: abcf::bs3::Store + 'static> abcf::Storage for ABCFManagerSimpleManagerSf<S> {
        fn rollback(&mut self, height: i64) -> Result<()> {
            self.mock.rollback(height)?;
            self.mock2.rollback(height)?;
            Ok(())
        }
        fn height(&self) -> Result<i64> {
            Ok(self.mock.height()?)
        }
        fn commit(&mut self) -> Result<()> {
            self.mock.commit()?;
            self.mock2.commit()?;
            Ok(())
        }
    }
    impl<S: abcf::bs3::Store + 'static> abcf::module::StorageTransaction
        for ABCFManagerSimpleManagerSf<S>
    {
        type Transaction<'a> = ABCFManagerSimpleManagerSfTx<'a, S>;
        type Cache = ABCFManagerSimpleManagerSfTxCache<S>;
        fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
            Self::Cache {
                mock: abcf::Stateful::<MockModule<S, Sha3_512>>::cache(tx.mock),
                mock2: abcf::Stateful::<MockModule<S, Sha3_512>>::cache(tx.mock2),
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
            self.mock2.execute(transaction.mock2);
        }
    }
    impl<S: abcf::bs3::Store + 'static>
        abcf::entry::Application<ABCFManagerSimpleManagerSl<S>, ABCFManagerSimpleManagerSf<S>>
        for SimpleManager<S>
    {
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn check_tx<'life0, 'life1, 'life2, 'life3, 'async_trait>(
            &'life0 mut self,
            context: &'life1 mut abcf::entry::TContext<
                ABCFManagerSimpleManagerSlTx<'life2, S>,
                ABCFManagerSimpleManagerSfTx<'life3, S>,
            >,
            _req: abcf::tm_protos::abci::RequestCheckTx,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = abcf::ModuleResult<abcf::module::types::ResponseCheckTx>,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            'life3: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    abcf::ModuleResult<abcf::module::types::ResponseCheckTx>,
                > {
                    return __ret;
                }
                let mut __self = self;
                let context = context;
                let _req = _req;
                let __ret: abcf::ModuleResult<abcf::module::types::ResponseCheckTx> =
                    {
                        use abcf::module::FromBytes;
                        use abcf::Application;
                        use abcf::Error;
                        use abcf::Module;
                        use std::collections::BTreeMap;
                        use std::convert::TryInto;
                        let req_tx = SimpleNodeTransaction::from_bytes(&_req.tx).map_err(|e| {
                            abcf::ModuleError {
                                namespace: String::from("abcf.manager"),
                                error: e,
                            }
                        })?;
                        let req_tx_ref = &req_tx;
                        let mut resp_check_tx = abcf::module::types::ResponseCheckTx::default();
                        let mut data_map = BTreeMap::new();
                        let name = __self.mock.metadata().name.to_string();
                        let tx = abcf::module::types::RequestCheckTx {
                            ty: _req.r#type,
                            tx: req_tx_ref.try_into().map_err(|e| abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            })?,
                        };
                        let ctx = abcf::manager::TContext {
                            events: abcf::entry::EventContext {
                                events: context.events.events,
                            },
                            stateful: &mut context.stateful.mock,
                            stateless: &mut context.stateless.mock,
                            deps: (),
                        };
                        let result = __self.mock.check_tx(ctx, &tx).await.map_err(|e| {
                            abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            }
                        })?;
                        data_map.insert(name.clone(), result.data);
                        resp_check_tx.gas_used += result.gas_used;
                        resp_check_tx.gas_wanted += result.gas_wanted;
                        let name = __self.mock2.metadata().name.to_string();
                        let tx = abcf::module::types::RequestCheckTx {
                            ty: _req.r#type,
                            tx: req_tx_ref.try_into().map_err(|e| abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            })?,
                        };
                        let ctx = abcf::manager::TContext {
                            events: abcf::entry::EventContext {
                                events: context.events.events,
                            },
                            stateful: &mut context.stateful.mock2,
                            stateless: &mut context.stateless.mock2,
                            deps: (),
                        };
                        let result = __self.mock2.check_tx(ctx, &tx).await.map_err(|e| {
                            abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            }
                        })?;
                        data_map.insert(name.clone(), result.data);
                        resp_check_tx.gas_used += result.gas_used;
                        resp_check_tx.gas_wanted += result.gas_wanted;
                        let data =
                            serde_json::to_vec(&data_map).map_err(|e| abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: Error::JsonError(e),
                            })?;
                        resp_check_tx.data = data;
                        Ok(resp_check_tx)
                    };
                #[allow(unreachable_code)]
                __ret
            })
        }
        /// Begin block.
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn begin_block<'life0, 'life1, 'async_trait>(
            &'life0 mut self,
            context: &'life1 mut abcf::entry::AContext<
                ABCFManagerSimpleManagerSl<S>,
                ABCFManagerSimpleManagerSf<S>,
            >,
            _req: abcf::module::types::RequestBeginBlock,
        ) -> ::core::pin::Pin<
            Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send + 'async_trait>,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                let mut __self = self;
                let context = context;
                let _req = _req;
                let _: () = {
                    use abcf::Application;
                    let ctx = abcf::manager::AContext {
                        events: abcf::entry::EventContext {
                            events: context.events.events,
                        },
                        stateful: &mut context.stateful.mock,
                        stateless: &mut context.stateless.mock,
                        deps: (),
                    };
                    __self.mock.begin_block(ctx, &_req).await;
                    let ctx = abcf::manager::AContext {
                        events: abcf::entry::EventContext {
                            events: context.events.events,
                        },
                        stateful: &mut context.stateful.mock2,
                        stateless: &mut context.stateless.mock2,
                        deps: (),
                    };
                    __self.mock2.begin_block(ctx, &_req).await;
                };
            })
        }
        /// Execute transaction on state.
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn deliver_tx<'life0, 'life1, 'life2, 'life3, 'async_trait>(
            &'life0 mut self,
            context: &'life1 mut abcf::entry::TContext<
                ABCFManagerSimpleManagerSlTx<'life2, S>,
                ABCFManagerSimpleManagerSfTx<'life3, S>,
            >,
            _req: abcf::tm_protos::abci::RequestDeliverTx,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<
                        Output = abcf::ModuleResult<abcf::module::types::ResponseDeliverTx>,
                    > + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            'life3: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) = ::core::option::Option::None::<
                    abcf::ModuleResult<abcf::module::types::ResponseDeliverTx>,
                > {
                    return __ret;
                }
                let mut __self = self;
                let context = context;
                let _req = _req;
                let __ret: abcf::ModuleResult<abcf::module::types::ResponseDeliverTx> =
                    {
                        use abcf::module::FromBytes;
                        use abcf::Application;
                        use abcf::Error;
                        use abcf::Module;
                        use std::collections::BTreeMap;
                        use std::convert::TryInto;
                        let req_tx = SimpleNodeTransaction::from_bytes(&_req.tx).map_err(|e| {
                            abcf::ModuleError {
                                namespace: String::from("abcf.manager"),
                                error: e,
                            }
                        })?;
                        let req_tx_ref = &req_tx;
                        let mut resp_deliver_tx = abcf::module::types::ResponseDeliverTx::default();
                        let mut data_map = BTreeMap::new();
                        let name = __self.mock.metadata().name.to_string();
                        let tx = abcf::module::types::RequestDeliverTx {
                            tx: req_tx_ref.try_into().map_err(|e| abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            })?,
                        };
                        let ctx = abcf::manager::TContext {
                            events: abcf::entry::EventContext {
                                events: context.events.events,
                            },
                            stateful: &mut context.stateful.mock,
                            stateless: &mut context.stateless.mock,
                            deps: (),
                        };
                        let result = __self.mock.deliver_tx(ctx, &tx).await.map_err(|e| {
                            abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            }
                        })?;
                        data_map.insert(name.clone(), result.data);
                        resp_deliver_tx.gas_used += result.gas_used;
                        resp_deliver_tx.gas_wanted += result.gas_wanted;
                        let name = __self.mock2.metadata().name.to_string();
                        let tx = abcf::module::types::RequestDeliverTx {
                            tx: req_tx_ref.try_into().map_err(|e| abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            })?,
                        };
                        let ctx = abcf::manager::TContext {
                            events: abcf::entry::EventContext {
                                events: context.events.events,
                            },
                            stateful: &mut context.stateful.mock2,
                            stateless: &mut context.stateless.mock2,
                            deps: (),
                        };
                        let result = __self.mock2.deliver_tx(ctx, &tx).await.map_err(|e| {
                            abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: e,
                            }
                        })?;
                        data_map.insert(name.clone(), result.data);
                        resp_deliver_tx.gas_used += result.gas_used;
                        resp_deliver_tx.gas_wanted += result.gas_wanted;
                        let data =
                            serde_json::to_vec(&data_map).map_err(|e| abcf::ModuleError {
                                namespace: String::from(name.clone()),
                                error: Error::JsonError(e),
                            })?;
                        resp_deliver_tx.data = data;
                        Ok(resp_deliver_tx)
                    };
                #[allow(unreachable_code)]
                __ret
            })
        }
        /// End Block.
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn end_block<'life0, 'life1, 'async_trait>(
            &'life0 mut self,
            context: &'life1 mut abcf::entry::AContext<
                ABCFManagerSimpleManagerSl<S>,
                ABCFManagerSimpleManagerSf<S>,
            >,
            _req: abcf::module::types::RequestEndBlock,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = abcf::module::types::ResponseEndBlock>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<abcf::module::types::ResponseEndBlock>
                {
                    return __ret;
                }
                let mut __self = self;
                let context = context;
                let _req = _req;
                let __ret: abcf::module::types::ResponseEndBlock = {
                    use abcf::Application;
                    let mut validator_updates_vec = Vec::new();
                    let mut resp_end_block = abcf::module::types::ResponseEndBlock::default();
                    let ctx = abcf::manager::AContext {
                        events: abcf::entry::EventContext {
                            events: context.events.events,
                        },
                        stateful: &mut context.stateful.mock,
                        stateless: &mut context.stateless.mock,
                        deps: (),
                    };
                    let resp = __self.mock.end_block(ctx, &_req).await;
                    resp.validator_updates.into_iter().for_each(|v| {
                        if !validator_updates_vec.contains(&v) {
                            validator_updates_vec.push(v);
                        }
                    });
                    resp_end_block.consensus_param_updates = resp.consensus_param_updates;
                    let ctx = abcf::manager::AContext {
                        events: abcf::entry::EventContext {
                            events: context.events.events,
                        },
                        stateful: &mut context.stateful.mock2,
                        stateless: &mut context.stateless.mock2,
                        deps: (),
                    };
                    let resp = __self.mock2.end_block(ctx, &_req).await;
                    resp.validator_updates.into_iter().for_each(|v| {
                        if !validator_updates_vec.contains(&v) {
                            validator_updates_vec.push(v);
                        }
                    });
                    resp_end_block.consensus_param_updates = resp.consensus_param_updates;
                    resp_end_block.validator_updates = validator_updates_vec;
                    resp_end_block
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
    impl<S: abcf::bs3::Store + 'static>
        abcf::entry::RPCs<ABCFManagerSimpleManagerSl<S>, ABCFManagerSimpleManagerSf<S>>
        for SimpleManager<S>
    {
        #[allow(
            clippy::let_unit_value,
            clippy::type_complexity,
            clippy::type_repetition_in_bounds,
            clippy::used_underscore_binding
        )]
        fn call<'life0, 'life1, 'life2, 'async_trait>(
            &'life0 mut self,
            ctx: &'life1 mut abcf::entry::RContext<
                ABCFManagerSimpleManagerSl<S>,
                ABCFManagerSimpleManagerSf<S>,
            >,
            method: &'life2 str,
            params: serde_json::Value,
        ) -> ::core::pin::Pin<
            Box<
                dyn ::core::future::Future<Output = abcf::ModuleResult<Option<serde_json::Value>>>
                    + ::core::marker::Send
                    + 'async_trait,
            >,
        >
        where
            'life0: 'async_trait,
            'life1: 'async_trait,
            'life2: 'async_trait,
            Self: 'async_trait,
        {
            Box::pin(async move {
                if let ::core::option::Option::Some(__ret) =
                    ::core::option::Option::None::<abcf::ModuleResult<Option<serde_json::Value>>>
                {
                    return __ret;
                }
                let mut __self = self;
                let ctx = ctx;
                let method = method;
                let params = params;
                let __ret: abcf::ModuleResult<Option<serde_json::Value>> = {
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
                        "mock" => {
                            let context = abcf::manager::RContext {
                                stateful: &ctx.stateful.mock,
                                stateless: &mut ctx.stateless.mock,
                                deps: (),
                            };
                            __self
                                .mock
                                .call(context, method, params)
                                .await
                                .map_err(|e| abcf::ModuleError {
                                    namespace: String::from("mock"),
                                    error: e,
                                })
                        }
                        "mock2" => {
                            let context = abcf::manager::RContext {
                                stateful: &ctx.stateful.mock2,
                                stateless: &mut ctx.stateless.mock2,
                                deps: (),
                            };
                            __self
                                .mock2
                                .call(context, method, params)
                                .await
                                .map_err(|e| abcf::ModuleError {
                                    namespace: String::from("mock2"),
                                    error: e,
                                })
                        }
                        _ => Err(abcf::ModuleError {
                            namespace: String::from("abcf.manager"),
                            error: abcf::Error::NoModule,
                        }),
                    }
                };
                #[allow(unreachable_code)]
                __ret
            })
        }
    }
}
fn main() {
    let body = async {};
    #[allow(clippy::expect_used)]
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(body)
}
