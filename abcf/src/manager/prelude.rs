use digest::Digest;

use crate::{module::StorageTransaction, Merkle, Storage};

pub trait ModuleStorage<D>
where
    D: Digest,
{
    type Stateless: Storage + StorageTransaction;

    type Stateful: Storage + StorageTransaction + Merkle<D>;
}
