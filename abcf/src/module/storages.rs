use bs3::Store;
use digest::{Digest, Output};

use crate::Result;

/// Define module's storage.
pub trait Storage<S: Store>: Send + Sync {
    type Transaction<'a>: Send;

    fn rollback(&mut self, height: i64) -> Result<()>;

    fn height(&self) -> Result<i64>;

    fn commit(&mut self) -> Result<()>;

    fn transaction(&self) -> Self::Transaction<'_>;

    fn execute(&mut self, transaction: Self::Transaction<'_>);
}

pub trait Merkle<D: Digest> {
    fn root(&self) -> Result<Output<D>>;
}
