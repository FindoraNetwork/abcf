#![feature(generic_associated_types)]

mod data;
mod event;
mod transaction;

use std::marker::PhantomData;

use abcf::{
    abci::{RequestBeginBlock, RequestEndBlock},
    manager::{AContext, TContext},
    module::{
        types::{
            RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx, ResponseEndBlock,
        },
        StorageTransaction,
    },
    Application, Result, Stateful, StatefulBatch, Stateless, StatelessBatch,
};
use bs3::model::{Map, Value};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// /// Module's Event
// #[derive(Clone, Debug, Deserialize, Serialize, Event)]
// pub struct Event1 {}
//

pub trait Config: Send + Sync + Debug + Clone + 'static {
    type Address: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    type Signature: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    type AssetCode: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    type PublicKey: Debug + Clone + Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static;

    type OutputId: Debug
        + Clone
        + Serialize
        + for<'de> Deserialize<'de>
        + PartialOrd
        + Ord
        + Send
        + Sync
        + 'static;
}

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct UTXOModule<C: Config> {
    marker: PhantomData<C>,
    #[stateful]
    pub unspent_outputs: Map<C::OutputId, data::TxOutput<C>>,
    #[stateless]
    pub marker: Value<u32>,
}

#[abcf::rpcs]
impl<C: Config + Sync + Send> UTXOModule<C> {}

/// Module's block logic.
#[abcf::application]
impl<C: Config + Sync + Send> Application for UTXOModule<C> {
    type Transaction = transaction::Transaction<C>;

    async fn check_tx(
        &mut self,
        _context: &mut TContext<StatelessBatch<'_, Self>, StatefulBatch<'_, Self>>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    async fn begin_block(
        &mut self,
        _context: &mut AContext<Stateless<Self>, Stateful<Self>>,
        _req: &RequestBeginBlock,
    ) {
        // use bs3::ValueStore;
        // let a = _context.stateless.sl_value.get();
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
impl<C: Config + Sync + Send> UTXOModule<C> {}
