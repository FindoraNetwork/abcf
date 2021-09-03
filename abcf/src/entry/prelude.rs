use alloc::boxed::Box;
use serde_json::Value;

use crate::{
    module::types::{
        RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx,
        ResponseDeliverTx, ResponseEndBlock,
    },
    ModuleResult,
};

use super::{Context, RPCContext};

#[async_trait::async_trait]
pub trait RPCs<Sl, Sf>: Send + Sync {
    async fn call(
        &mut self,
        ctx: &mut RPCContext<Sl, Sf>,
        method: &str,
        params: Value,
    ) -> ModuleResult<Option<Value>>;
}

/// This trait define module's main blockchain logic.
#[async_trait::async_trait]
pub trait Application<Sl, Sf>: Send + Sync
where
    Sl: Sync + Send,
    Sf: Sync + Send,
{
    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx(
        &mut self,
        _context: &mut Context<Sl, Sf>,
        _req: &RequestCheckTx,
    ) -> ModuleResult<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Begin block.
    async fn begin_block(&mut self, _context: &mut Context<Sl, Sf>, _req: &RequestBeginBlock) {}

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut Context<Sl, Sf>,
        _req: &RequestDeliverTx,
    ) -> ModuleResult<ResponseDeliverTx> {
        Ok(Default::default())
    }

    /// End Block.
    async fn end_block(
        &mut self,
        _context: &mut Context<Sl, Sf>,
        _req: &RequestEndBlock,
    ) -> ResponseEndBlock {
        Default::default()
    }
}
