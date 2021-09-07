use digest::{Digest, Output};

use crate::Result;

/// Define module's storage.
pub trait Storage: Send + Sync {
    fn rollback(&mut self, height: i64) -> Result<()>;

    fn height(&self) -> Result<i64>;

    fn commit(&mut self) -> Result<()>;
}

pub trait StorageTransaction {
    type Transaction: Send;

    fn transaction(&self) -> Self::Transaction;

    fn execute(&mut self, transaction: Self::Transaction);
}

pub trait Merkle<D: Digest> {
    fn root(&self) -> Result<Output<D>>;
}
