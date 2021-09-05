use alloc::{boxed::Box, vec::Vec};
use bs3::Store;
use serde_json::Value;

use crate::{
    module::types::{
        RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx,
        ResponseDeliverTx, ResponseEndBlock,
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
pub trait Application<S, Sl, Sf>: Send + Sync
where
    S: Store,
    Sl: Storage<S>,
    Sf: Storage<S>,
{
    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx(
        &mut self,
        _context: &mut TContext<S, Sl, Sf>,
        _req: RequestCheckTx,
    ) -> ModuleResult<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Begin block.
    async fn begin_block(&mut self, _context: &mut AContext<Sl, Sf>, _req: RequestBeginBlock) {}

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TContext<S, Sl, Sf>,
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

pub trait Tree {
    fn get(&self, key: &str, height: i64) -> ModuleResult<Vec<u8>>;
}
