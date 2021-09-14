use alloc::boxed::Box;

pub mod types;
pub use types::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, ResponseCheckTx,
    ResponseDeliverTx, ResponseEndBlock,
};

use crate::{
    manager::{AContext, TContext},
    Result, Storage,
};

use super::StorageTransaction;

/// This trait define module's main blockchain logic.
#[async_trait::async_trait]
pub trait Application<Sl, Sf>: Send + Sync
where
    Sl: Storage + StorageTransaction,
    Sf: Storage + StorageTransaction,
{
    type Transaction: Default + Send + Sync + Into<Self::Transaction>;

    /// Define how to check transaction.
    ///
    /// In this function, do some lightweight check for transaction, for example: check signature,
    /// check balance and so on.
    /// This method will be called at external user or another node.
    async fn check_tx(
        &mut self,
        _context: &mut TContext<Sl::Transaction<'_>, Sf::Transaction<'_>>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Begin block.
    async fn begin_block(&mut self, _context: &mut AContext<Sl, Sf>, _req: &RequestBeginBlock) {}

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TContext<Sl::Transaction<'_>, Sf::Transaction<'_>>,
        _req: &RequestDeliverTx<Self::Transaction>,
    ) -> Result<ResponseDeliverTx> {
        Ok(Default::default())
    }

    /// End Block.
    async fn end_block(
        &mut self,
        _context: &mut AContext<Sl, Sf>,
        _req: &RequestEndBlock,
    ) -> ResponseEndBlock {
        Default::default()
    }
}
