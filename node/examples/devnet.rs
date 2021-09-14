#![feature(generic_associated_types)]

/// Running in shell
///
/// ``` bash
/// $ cargo run --example devnet
/// ```
use abcf::{module::StorageTransaction, Application, Event};
use bs3::model::{Map, Value};
use serde::{Deserialize, Serialize};
use sha3::Sha3_512;

/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct Event1 {}

#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct MockModule {
    // /// In memory.
    pub inner: u32,
    #[stateful]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

#[abcf::rpcs]
impl MockModule {}

/// Module's block logic.
#[abcf::application]
impl Application for MockModule {
    type Transaction = MockTransaction;
}

pub struct MockTransaction {}

impl Default for MockTransaction {
    fn default() -> Self {
        MockTransaction {}
    }
}

/// Module's methods.
#[abcf::methods]
impl MockModule {}

pub struct SimpleNode<S: abcf::bs3::Store + 'static> {
    pub mock: MockModule<S>,
    pub mock2: MockModule<S>,
}

impl<S> abcf::Module for SimpleNode<S>
where
    S: abcf::bs3::Store + 'static,
{
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

pub struct SimpleNodeSl<S: abcf::bs3::Store + 'static> {
    pub mock: abcf::Stateless<MockModule<S>>,
    pub mock2: abcf::Stateless<MockModule<S>>,
}

impl<S> abcf::entry::Tree for SimpleNodeSl<S>
where
    S: abcf::bs3::Store,
{
    fn get(&self, _key: &str, _height: i64) -> abcf::ModuleResult<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl<S> abcf::Storage for SimpleNodeSl<S>
where
    S: abcf::bs3::Store,
{
    fn rollback(&mut self, height: i64) -> abcf::Result<()> {
        self.mock.rollback(height)?;
        self.mock2.rollback(height)?;
        Ok(())
    }

    fn height(&self) -> abcf::Result<i64> {
        let mock = self.mock.height()?;
        Ok(mock)
    }

    fn commit(&mut self) -> abcf::Result<()> {
        self.mock.commit()?;
        self.mock2.commit()?;
        Ok(())
    }
}

pub struct SimpleNodeSlTx<'a, S: abcf::bs3::Store + 'static> {
    pub mock: abcf::StatelessBatch<'a, MockModule<S>>,
    pub mock2: abcf::StatelessBatch<'a, MockModule<S>>,
}

pub struct SimpleNodeSlTxCache<S: abcf::bs3::Store + 'static> {
    pub mock: abcf::StatelessCache<MockModule<S>>,
    pub mock2: abcf::StatelessCache<MockModule<S>>,
}

impl<S> StorageTransaction for SimpleNodeSl<S>
where
    S: abcf::bs3::Store,
{
    type Transaction<'a> = SimpleNodeSlTx<'a, S>;

    type Cache = SimpleNodeSlTxCache<S>;

    fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
        Self::Cache {
            mock: abcf::Stateless::<MockModule<S>>::cache(tx.mock),
            mock2: abcf::Stateless::<MockModule<S>>::cache(tx.mock2),
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

pub struct SimpleNodeSf<S: abcf::bs3::Store + 'static> {
    pub mock: abcf::Stateful<MockModule<S>>,
    pub mock2: abcf::Stateful<MockModule<S>>,
}

impl<S> abcf::entry::Tree for SimpleNodeSf<S>
where
    S: abcf::bs3::Store,
{
    fn get(&self, _key: &str, _height: i64) -> abcf::ModuleResult<Vec<u8>> {
        Ok(Vec::new())
    }
}

impl<S> abcf::module::Merkle<Sha3_512> for SimpleNodeSf<S>
where
    S: abcf::bs3::Store,
{
    fn root(&self) -> abcf::Result<digest::Output<Sha3_512>> {
        Ok(Default::default())
    }
}

impl<S> abcf::Storage for SimpleNodeSf<S>
where
    S: abcf::bs3::Store,
{
    fn rollback(&mut self, height: i64) -> abcf::Result<()> {
        self.mock.rollback(height)?;
        self.mock2.rollback(height)?;
        Ok(())
    }

    fn height(&self) -> abcf::Result<i64> {
        let mock = self.mock.height()?;
        Ok(mock)
    }

    fn commit(&mut self) -> abcf::Result<()> {
        self.mock.commit()?;
        self.mock2.commit()?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleNodeTransaction {
    pub v: u64,
}

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

impl Into<MockTransaction> for SimpleNodeTransaction {
    fn into(self) -> MockTransaction {
        MockTransaction {}
    }
}

pub struct SimpleNodeSfTx<'a, S: abcf::bs3::Store + 'static> {
    pub mock: abcf::StatefulBatch<'a, MockModule<S>>,
    pub mock2: abcf::StatefulBatch<'a, MockModule<S>>,
}

pub struct SimpleNodeSfTxCache<S: abcf::bs3::Store + 'static> {
    pub mock: abcf::StatefulCache<MockModule<S>>,
    pub mock2: abcf::StatefulCache<MockModule<S>>,
}

impl<S> StorageTransaction for SimpleNodeSf<S>
where
    S: abcf::bs3::Store,
{
    type Transaction<'a> = SimpleNodeSfTx<'a, S>;

    type Cache = SimpleNodeSfTxCache<S>;

    fn cache(tx: Self::Transaction<'_>) -> Self::Cache {
        Self::Cache {
            mock: abcf::Stateful::<MockModule<S>>::cache(tx.mock),
            mock2: abcf::Stateful::<MockModule<S>>::cache(tx.mock2),
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

#[async_trait::async_trait]
impl<S> abcf::entry::Application<SimpleNodeSl<S>, SimpleNodeSf<S>> for SimpleNode<S>
where
    S: abcf::bs3::Store + 'static,
{
    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx(
        &mut self,
        context: &mut abcf::entry::TContext<SimpleNodeSlTx<'_, S>, SimpleNodeSfTx<'_, S>>,
        _req: abcf::abci::RequestCheckTx,
    ) -> abcf::ModuleResult<abcf::module::types::ResponseCheckTx> {
        use abcf::module::FromBytes;

        let mut ctx = abcf::manager::TContext {
            events: abcf::entry::EventContext {
                events: context.events.events,
            },
            stateful: &mut context.stateful.mock,
            stateless: &mut context.stateless.mock,
        };

        let req_tx =
            SimpleNodeTransaction::from_bytes(&_req.tx).map_err(|e| abcf::ModuleError {
                namespace: String::from("mock"),
                error: e,
            })?;

        let tx = abcf::module::types::RequestCheckTx {
            ty: _req.r#type,
            tx: req_tx.into(),
        };

        let result = self
            .mock
            .check_tx(&mut ctx, &tx)
            .await
            .map_err(|e| abcf::ModuleError {
                namespace: String::from("mock"),
                error: e,
            })?;

        Ok(result)
    }

    /// Begin block.
    async fn begin_block(
        &mut self,
        context: &mut abcf::entry::AContext<SimpleNodeSl<S>, SimpleNodeSf<S>>,
        _req: abcf::module::types::RequestBeginBlock,
    ) {
        let mut ctx = abcf::manager::AContext {
            events: abcf::entry::EventContext {
                events: context.events.events,
            },
            stateful: &mut context.stateful.mock,
            stateless: &mut context.stateless.mock,
        };

        self.mock.begin_block(&mut ctx, &_req).await;
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        context: &mut abcf::entry::TContext<SimpleNodeSlTx<'_, S>, SimpleNodeSfTx<'_, S>>,
        _req: abcf::abci::RequestDeliverTx,
    ) -> abcf::ModuleResult<abcf::module::types::ResponseDeliverTx> {
        use abcf::module::FromBytes;

        let mut ctx = abcf::manager::TContext {
            events: abcf::entry::EventContext {
                events: context.events.events,
            },
            stateful: &mut context.stateful.mock,
            stateless: &mut context.stateless.mock,
        };

        let req_tx =
            SimpleNodeTransaction::from_bytes(&_req.tx).map_err(|e| abcf::ModuleError {
                namespace: String::from("mock"),
                error: e,
            })?;

        let tx = abcf::module::types::RequestDeliverTx { tx: req_tx.into() };

        let result = self
            .mock
            .deliver_tx(&mut ctx, &tx)
            .await
            .map_err(|e| abcf::ModuleError {
                namespace: String::from("mock"),
                error: e,
            })?;

        Ok(result)
    }

    /// End Block.
    async fn end_block(
        &mut self,
        context: &mut abcf::entry::AContext<SimpleNodeSl<S>, SimpleNodeSf<S>>,
        _req: abcf::module::types::RequestEndBlock,
    ) -> abcf::module::types::ResponseEndBlock {
        let mut ctx = abcf::manager::AContext {
            events: abcf::entry::EventContext {
                events: context.events.events,
            },
            stateful: &mut context.stateful.mock,
            stateless: &mut context.stateless.mock,
        };

        self.mock.end_block(&mut ctx, &_req).await
    }
}

#[async_trait::async_trait]
impl<S> abcf::entry::RPCs<SimpleNodeSl<S>, SimpleNodeSf<S>> for SimpleNode<S>
where
    S: abcf::bs3::Store + 'static,
{
    async fn call(
        &mut self,
        ctx: &mut abcf::entry::RContext<SimpleNodeSl<S>, SimpleNodeSf<S>>,
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
            namespace: String::from("abcf.managing"),
            error: abcf::Error::QueryPathFormatError,
        })?;

        match module_name {
            "mock" => {
                let mut context = abcf::manager::RContext {
                    stateful: &ctx.stateful.mock,
                    stateless: &mut ctx.stateless.mock,
                };

                self.mock
                    .call(&mut context, method, params)
                    .await
                    .map_err(|e| abcf::ModuleError {
                        namespace: String::from("mock"),
                        error: e,
                    })
            }
            _ => Err(abcf::ModuleError {
                namespace: String::from("abcf.manager"),
                error: abcf::Error::NoModule,
            }),
        }
    }
}

fn main() {
    env_logger::init();
    use bs3::backend::MemoryBackend;

    let mock = MockModule::new(1);

    let mock2 = MockModule::new(2);

    let simple_node = SimpleNode::<MemoryBackend> { mock, mock2 };

    let stateless = SimpleNodeSl {
        mock: abcf::Stateless::<MockModule<MemoryBackend>> {
            sl_map: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            sl_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
        },
        mock2: abcf::Stateless::<MockModule<MemoryBackend>> {
            sl_map: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            sl_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
        },
    };

    let stateful = SimpleNodeSf {
        mock: abcf::Stateful::<MockModule<MemoryBackend>> {
            sf_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
        },
        mock2: abcf::Stateful::<MockModule<MemoryBackend>> {
            sf_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
        },
    };

    let entry = abcf::entry::Node::new(stateless, stateful, simple_node);
    let node = abcf_node::Node::new(entry, "./target/abcf").unwrap();
    node.start().unwrap();
    std::thread::park();
}
