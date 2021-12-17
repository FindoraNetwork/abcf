use alloc::boxed::Box;

pub mod types;
pub use types::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx,
    ResponseDeliverTx, ResponseEndBlock,
};

use crate::{
    manager::{Dependence, ModuleStorage},
    AppContext, Result, TxnContext,
};

// use super::StorageTransaction;

/// This trait define module's main blockchain logic.
#[async_trait::async_trait]
pub trait Application: Send + Sync + Sized + ModuleStorage + Dependence {
    type Transaction: Default + Send + Sync;

    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Begin block.
    async fn begin_block(&mut self, _context: &mut AppContext<'_, Self>, _req: &RequestBeginBlock) {
    }

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TxnContext<'_, Self>,
        _req: &RequestDeliverTx<Self::Transaction>,
    ) -> Result<ResponseDeliverTx> {
        Ok(Default::default())
    }

    /// End Block.
    async fn end_block(
        &mut self,
        _context: &mut AppContext<'_, Self>,
        _req: &RequestEndBlock,
    ) -> ResponseEndBlock {
        Default::default()
    }
}
