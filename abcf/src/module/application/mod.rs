use alloc::boxed::Box;

pub mod types;
pub use types::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx,
    ResponseDeliverTx, ResponseEndBlock,
};

use crate::{
    manager::{ModuleStorage, ModuleStorageDependence},
    AppContext, Result, TxnContext,
};

// use super::StorageTransaction;

/// This trait define module's main blockchain logic.
#[async_trait::async_trait]
pub trait Application: Send + Sync {
    type Transaction: Default + Send + Sync;

    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx<'a>(
        &mut self,
        _context: TxnContext<'a, Self>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> Result<ResponseCheckTx>
    where
        Self: ModuleStorageDependence<'a> + ModuleStorage,
    {
        Ok(Default::default())
    }

    /// Begin block.
    async fn begin_block<'a>(
        &mut self,
        _context: AppContext<'a, Self>,
        _req: &RequestBeginBlock,
    ) where
        Self: ModuleStorageDependence<'a> + ModuleStorage,
    {
    }

    /// Execute transaction on state.
    async fn deliver_tx<'a>(
        &mut self,
        _context: TxnContext<'a, Self>,
        _req: &RequestDeliverTx<Self::Transaction>,
    ) -> Result<ResponseDeliverTx>
    where
        Self: ModuleStorageDependence<'a> + ModuleStorage,
    {
        Ok(Default::default())
    }

    /// End Block.
    async fn end_block<'a>(
        &mut self,
        _context: AppContext<'a, Self>,
        _req: &RequestEndBlock,
    ) -> ResponseEndBlock
    where
        Self: ModuleStorageDependence<'a> + ModuleStorage,
    {
        Default::default()
    }
}
