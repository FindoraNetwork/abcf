use alloc::boxed::Box;
pub use tm_protos::abci::{RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock};

pub mod types;
pub use types::{ResponseCheckTx, ResponseDeliverTx, ResponseEndBlock};

use crate::{Result, Storage, manager::{AContext, TContext}};

use super::StorageTransaction;

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
        _context: &mut TContext<Sl::Transaction, Sf::Transaction>,
        _req: &RequestCheckTx,
    ) -> Result<ResponseCheckTx> {
        Ok(Default::default())
    }

    /// Begin block.
    async fn begin_block(&mut self, _context: &mut AContext<Sl, Sf>, _req: &RequestBeginBlock) {}

    /// Execute transaction on state.
    async fn deliver_tx(
        &mut self,
        _context: &mut TContext<Sl::Transaction, Sf::Transaction>,
        _req: &RequestDeliverTx,
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
