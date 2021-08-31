use bs3::Store;

use crate::Result;

/// Define module's storage.
pub trait Storage<S: Store> {
    type Transaction;

    fn rollback(&mut self, height: u64) -> Result<()>;

    fn height(&self) -> u64;

    fn commit(&mut self) -> Result<()>;

    fn transaction(&mut self) -> Self::Transaction;

    fn execute(&mut self, transaction: Self::Transaction);
}

