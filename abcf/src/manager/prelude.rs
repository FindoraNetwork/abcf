use bs3::Store;
use digest::Digest;

use crate::{Merkle, Storage};

pub trait ModuleStorage<D>
where
    D: Digest,
{
    type Stateless: Storage;

    type Stateful: Storage + Merkle<D>;
}
