#![feature(generic_associated_types)]

use abcf::{
    bs3::{
        merkle::append_only::AppendOnlyMerkle,
        model::{Map, Value},
    },
    module::types::{RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx},
    Application, TxnContext,
};

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
    type Transaction = ();
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
    type Transaction = ();

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

#[tokio::main]
async fn main() {}
