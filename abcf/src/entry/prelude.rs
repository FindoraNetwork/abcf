use alloc::{boxed::Box, vec::Vec};
use serde_json::Value;
use tm_protos::abci::{RequestCheckTx, RequestDeliverTx};

use crate::{
    manager::ModuleStorage,
    module::types::{
        RequestBeginBlock, RequestEndBlock, ResponseCheckTx, ResponseDeliverTx, ResponseEndBlock,
    },
    ModuleResult,
};

use super::{AppContext, RPCContext, TxnContext};

#[async_trait::async_trait]
pub trait RPCs: Send + Sync
where
    Self: ModuleStorage + Sized,
{
    async fn call(
        &mut self,
        ctx: &mut RPCContext<'_, Self>,
        method: &str,
        params: Value,
    ) -> ModuleResult<Option<Value>>;
}

/// This trait define module's main blockchain logic.
#[async_trait::async_trait]
pub trait Application: Send + Sync
where
    Self: ModuleStorage + Sized,
{
    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: RequestCheckTx,
    ) -> ModuleResult<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Begin block.
    async fn begin_block(&mut self, _context: &mut AppContext<'_, Self>, _req: RequestBeginBlock) {}

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: RequestDeliverTx,
    ) -> ModuleResult<ResponseDeliverTx> {
        Ok(Default::default())
    }

    /// End Block.
    async fn end_block(
        &mut self,
        _context: &mut AppContext<'_, Self>,
        _req: RequestEndBlock,
    ) -> ResponseEndBlock {
        Default::default()
    }
}

pub trait Tree {
    fn get(&self, key: &str, height: i64) -> ModuleResult<Vec<u8>>;
}

impl Tree for () {
    fn get(&self, _key: &str, _height: i64) -> ModuleResult<Vec<u8>> {
        Ok(Vec::new())
    }
}
