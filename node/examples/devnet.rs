/// Running in shell
///
/// ``` bash
/// $ cargo run --example devnet
/// ```
use abcf::{
    entry::Tree, module::StorageTransaction, Application, Error, Event, Merkle, ModuleError,
    Storage,
};
use bs3::model::{Map, Value};

/// Module's Event
#[derive(Debug, Event)]
pub struct Event1 {}

#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct MockModule {
    /// In memory.
    pub inner: u32,
    #[stateful]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

/// Module's rpc.
#[abcf::rpcs]
impl MockModule {}

/// Module's block logic.
#[abcf::application]
impl Application<EmptyStorage, EmptyStorage> for MockModule {}

/// Module's methods.
impl MockModule {}

// these code need auto generate.

mod __abcf_storage_mockmodule {
    use super::*;
    use abcf::Result;
    pub struct ABCFModuleMockModuleSl<S>
    where
        S: abcf::bs3::Store,
    {
        pub sl_value: abcf::bs3::SnapshotableStorage<S, Value<u32>>,
        pub sl_map: abcf::bs3::SnapshotableStorage<S, Map<i32, u32>>,
    }
    pub struct ABCFModuleMockModuleSlTx<'a, S>
    where
        S: abcf::bs3::Store,
    {
        pub sl_value: abcf::bs3::Transaction<'a, S, Value<u32>>,
        pub sl_map: abcf::bs3::Transaction<'a, S, Map<i32, u32>>,
    }
    impl<S> abcf::Storage for ABCFModuleMockModuleSl<S>
    where
        S: abcf::bs3::Store,
    {
        fn rollback(&mut self, height: i64) -> Result<()> {
            self.sl_value.rollback(height)?;
            self.sl_map.rollback(height)?;
            Ok(())
        }
        fn height(&self) -> Result<i64> {
            Ok(0)
        }
        fn commit(&mut self) -> Result<()> {
            self.sl_value.commit()?;
            self.sl_map.commit()?;
            Ok(())
        }
    }
    pub struct ABCFModuleMockModuleSf<S>
    where
        S: abcf::bs3::Store,
    {
        pub sf_value: abcf::bs3::SnapshotableStorage<S, Value<u32>>,
    }
}

pub struct SimpleNode {
    pub mock: MockModule,
}

impl abcf::Module for SimpleNode {
    fn metadata(&self) -> abcf::ModuleMetadata<'_> {
        abcf::ModuleMetadata {
            name: "simple_node",
            module_type: abcf::ModuleType::Manager,
            version: 1,
            impl_version: "0.1",
            genesis: abcf::Genesis { target_height: 0 },
        }
    }
}

pub struct EmptyStorage {}

impl EmptyStorage {
    pub fn new() -> Self {
        EmptyStorage {}
    }
}

impl Storage for EmptyStorage {
    fn height(&self) -> abcf::Result<i64> {
        Ok(0)
    }

    fn commit(&mut self) -> abcf::Result<()> {
        Ok(())
    }

    fn rollback(&mut self, _height: i64) -> abcf::Result<()> {
        Ok(())
    }
}

impl StorageTransaction for EmptyStorage {
    type Transaction = ();

    fn transaction(&self) -> Self::Transaction {
        ()
    }

    fn execute(&mut self, _transaction: Self::Transaction) {}
}

impl Merkle<sha3::Sha3_512> for EmptyStorage {
    fn root(&self) -> abcf::Result<digest::Output<sha3::Sha3_512>> {
        Ok(Default::default())
    }
}

impl Tree for EmptyStorage {
    fn get(&self, _key: &str, _height: i64) -> abcf::ModuleResult<Vec<u8>> {
        Ok(Vec::new())
    }
}

type StatelessTx = <EmptyStorage as StorageTransaction>::Transaction;
type StatefulTx = <EmptyStorage as StorageTransaction>::Transaction;

#[abcf::application]
impl abcf::entry::Application<EmptyStorage, EmptyStorage> for SimpleNode {
    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx(
        &mut self,
        context: &mut abcf::entry::TContext<StatelessTx, StatefulTx>,
        _req: abcf::module::types::RequestCheckTx,
    ) -> abcf::ModuleResult<abcf::module::types::ResponseCheckTx> {
        let mut ctx = abcf::manager::TContext {
            events: abcf::entry::EventContext {
                events: context.events.events,
            },
            stateful: context.stateful,
            stateless: context.stateless,
        };

        let result = self
            .mock
            .check_tx(&mut ctx, &_req)
            .await
            .map_err(|e| ModuleError {
                namespace: String::from("mock"),
                error: e,
            })?;

        Ok(result)
    }

    /// Begin block.
    async fn begin_block(
        &mut self,
        context: &mut abcf::entry::AContext<EmptyStorage, EmptyStorage>,
        _req: abcf::module::types::RequestBeginBlock,
    ) {
        let mut ctx = abcf::manager::AContext {
            events: abcf::entry::EventContext {
                events: context.events.events,
            },
            stateful: context.stateful,
            stateless: context.stateless,
        };
        self.mock.begin_block(&mut ctx, &_req).await;
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        context: &mut abcf::entry::TContext<StatelessTx, StatefulTx>,
        _req: abcf::module::types::RequestDeliverTx,
    ) -> abcf::ModuleResult<abcf::module::types::ResponseDeliverTx> {
        let mut ctx = abcf::manager::TContext {
            events: abcf::entry::EventContext {
                events: context.events.events,
            },
            stateful: context.stateful,
            stateless: context.stateless,
        };

        let result = self
            .mock
            .deliver_tx(&mut ctx, &_req)
            .await
            .map_err(|e| ModuleError {
                namespace: String::from("mock"),
                error: e,
            })?;

        Ok(result)
    }

    /// End Block.
    async fn end_block(
        &mut self,
        context: &mut abcf::entry::AContext<EmptyStorage, EmptyStorage>,
        _req: abcf::module::types::RequestEndBlock,
    ) -> abcf::module::types::ResponseEndBlock {
        let mut ctx = abcf::manager::AContext {
            events: abcf::entry::EventContext {
                events: context.events.events,
            },
            stateful: context.stateful,
            stateless: context.stateless,
        };
        self.mock.end_block(&mut ctx, &_req).await
    }
}

#[abcf::application]
impl abcf::entry::RPCs<EmptyStorage, EmptyStorage> for SimpleNode {
    async fn call(
        &mut self,
        ctx: &mut abcf::entry::RContext<EmptyStorage, EmptyStorage>,
        method: &str,
        params: serde_json::Value,
    ) -> abcf::ModuleResult<Option<serde_json::Value>> {
        use abcf::RPCs;
        let mut paths = method.split("/");
        let module_name = paths.next().ok_or(ModuleError {
            namespace: String::from("abcf.manager"),
            error: Error::QueryPathFormatError,
        })?;

        let method = paths.next().ok_or(ModuleError {
            namespace: String::from("abcf.managing"),
            error: Error::QueryPathFormatError,
        })?;

        let mut context = abcf::manager::RContext {
            stateful: ctx.stateful,
            stateless: ctx.stateless,
        };

        match module_name {
            "mock" => self
                .mock
                .call(&mut context, method, params)
                .await
                .map_err(|e| ModuleError {
                    namespace: String::from("mock"),
                    error: e,
                }),
            _ => Err(ModuleError {
                namespace: String::from("abcf.manager"),
                error: Error::NoModule,
            }),
        }
    }
}

fn main() {
    env_logger::init();
    let mock = MockModule { inner: 0 };

    let simple_node = SimpleNode { mock };

    let entry = abcf::entry::Node::new(EmptyStorage::new(), EmptyStorage::new(), simple_node);
    let node = abcf_node::Node::new(entry, "./target/abcf").unwrap();
    node.start().unwrap();
    std::thread::park();
}
