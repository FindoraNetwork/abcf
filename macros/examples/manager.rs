#![feature(generic_associated_types)]

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
#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct DepModule {
    pub inner: u32,
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

#[abcf::rpcs]
impl DepModule {}

#[abcf::application]
impl Application for DepModule {
    type Transaction = DepsTransaction;
}

#[derive(Default)]
pub struct DepsTransaction {}

impl TryFrom<&SimpleNodeTransaction> for DepsTransaction {
    type Error = abcf::Error;

    fn try_from(_value: &SimpleNodeTransaction) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
#[dependence(dep_module = "DepModule")]
pub struct MockModule {
    pub inner: u32,
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

#[abcf::rpcs]
impl MockModule {}

#[abcf::application]
impl Application for MockModule {
    type Transaction = MockTransaction;

    async fn check_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    async fn deliver_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        Ok(Default::default())
    }
}

#[derive(Default)]
pub struct MockTransaction {}

#[derive(Default, Serialize, Deserialize)]
pub struct SimpleNodeTransaction {
    pub v: u64,
}

impl abcf::Transaction for SimpleNodeTransaction {}

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

#[abcf::manager(
    name = "simple_node",
    digest = "Sha3_512",
    version = 0,
    impl_version = "0.1.0",
    transaction = "SimpleNodeTransaction"
)]
pub struct SimpleManager {
    pub mock: DepModule,
    #[dependence(dep_module = "mock")]
    pub mock2: MockModule,
}
#[tokio::main]
async fn main() {}
