use bs3::Store;
use digest::Digest;

use crate::{Merkle, Storage};

pub trait ModuleStorage<S, D>
where
    S: Store,
    D: Digest,
{
    type Stateless: Storage<S>;

    type Stateful: Storage<S> + Merkle<D>;
}
