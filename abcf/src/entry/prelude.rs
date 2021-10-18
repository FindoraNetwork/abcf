use alloc::{boxed::Box, vec::Vec};
use serde_json::Value;
use tm_protos::abci::{RequestCheckTx, RequestDeliverTx};

use crate::{
    module::{
        types::{
            RequestBeginBlock, RequestEndBlock, ResponseCheckTx, ResponseDeliverTx,
            ResponseEndBlock,
        },
        StorageTransaction,
    },
    ModuleResult, Storage,
};

use super::{context::TContext, AContext, RContext};

#[async_trait::async_trait]
pub trait RPCs<Sl, Sf>: Send + Sync {
    async fn call(
        &mut self,
        ctx: &mut RContext<Sl, Sf>,
        method: &str,
        params: Value,
    ) -> ModuleResult<Option<Value>>;
}

/// This trait define module's main blockchain logic.
#[async_trait::async_trait]
pub trait Application<Sl, Sf>: Send + Sync
where
    Sl: Storage + StorageTransaction,
    Sf: Storage + StorageTransaction,
{
    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx(
        &mut self,
        _context: &mut TContext<Sl::Transaction<'_>, Sf::Transaction<'_>>,
        _req: RequestCheckTx,
    ) -> ModuleResult<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Begin block.
    async fn begin_block(&mut self, _context: &mut AContext<Sl, Sf>, _req: RequestBeginBlock) {}

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TContext<Sl::Transaction<'_>, Sf::Transaction<'_>>,
        _req: RequestDeliverTx,
    ) -> ModuleResult<ResponseDeliverTx> {
        Ok(Default::default())
    }

    /// End Block.
    async fn end_block(
        &mut self,
        _context: &mut AContext<Sl, Sf>,
        _req: RequestEndBlock,
    ) -> ResponseEndBlock {
        Default::default()
    }
}

/// Cache
#[async_trait::async_trait]
pub trait CacheSender {
    /// trig abci method begin_block
    async fn begin_block(&self, req: RequestBeginBlock);
    async fn deliver_tx(&self, req: RequestDeliverTx);
    async fn end_block(&self, req: RequestEndBlock);
}

pub trait EntryCache {
    type Sender: CacheSender + Send + Sync;

    fn set_cache(&mut self, cache: Self::Sender);
}

pub trait CacheABCI {
    fn begin_block(&mut self, req: RequestBeginBlock);
    fn deliver_tx(&mut self, req: RequestDeliverTx);
    fn end_block(&mut self, req: RequestEndBlock);
}

pub trait Tree {
    fn get(&self, key: &str, height: i64) -> ModuleResult<Vec<u8>>;
}

impl Tree for () {
    fn get(&self, _key: &str, _height: i64) -> ModuleResult<Vec<u8>> {
        Ok(Vec::new())
    }
}
