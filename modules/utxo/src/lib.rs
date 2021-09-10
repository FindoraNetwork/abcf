#![feature(generic_associated_types)]

use abcf::{
    abci::{RequestBeginBlock, RequestEndBlock},
    manager::{AContext, TContext},
    module::types::{
        RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx, ResponseEndBlock,
    },
    Application, Event, Result, Stateful, StatefulBatch, Stateless, StatelessBatch,
};
use bs3::{model::{Map, Value}};
use serde::{Deserialize, Serialize};

/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct Event1 {}

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct UTXOModule<C: Config> {
    pub inner: u32,
    #[stateful]
    pub sf_value: Value<u32>,
    #[stateful]
    pub sf_value1: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

#[abcf::rpcs]
impl UTXOModule {

}

/// Module's block logic.
#[abcf::application]
impl Application for UTXOModule {
    type Transaction = Vec<u8>;

    async fn check_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> Result<ResponseCheckTx> {
        let e = Event1 {};
        _context.events.emmit(e).unwrap();
        Ok(Default::default())
    }

    async fn begin_block(
        &mut self,
        _context: &mut AContext<Stateless<Self>, Stateful<Self>>,
        _req: &RequestBeginBlock,
    ) {
        use bs3::ValueStore;
        let a = _context.stateless.sl_value.get();
    }

    async fn deliver_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        _req: &RequestDeliverTx<Self::Transaction>,
    ) -> Result<ResponseDeliverTx> {
        Ok(Default::default())
    }

    async fn end_block(
        &mut self,
        _context: &mut AContext<Stateless<Self>, Stateful<Self>>,
        _req: &RequestEndBlock,
    ) -> ResponseEndBlock {
        Default::default()
    }
}

/// Module's methods.
#[abcf::methods]
impl UTXOModule {}
