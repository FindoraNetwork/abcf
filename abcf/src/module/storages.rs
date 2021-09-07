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

impl Storage for () {
    fn rollback(&mut self, _height: i64) -> Result<()> {
        Ok(())
    }

    fn height(&self) -> Result<i64> {
        Ok(0)
    }

    fn commit(&mut self) -> Result<()> {
        Ok(())
    }
}

impl StorageTransaction for () {
    type Transaction = ();

    fn transaction(&self) -> Self::Transaction {
        ()
    }

    fn execute(&mut self, _transaction: Self::Transaction) {}
}

// impl Merkle<>
