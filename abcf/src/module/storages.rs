use core::fmt::Debug;
use alloc::vec::Vec;
use serde::{Deserialize, Serialize};

pub trait Value: Clone + Debug + Default + Serialize + for<'de> Deserialize<'de> {}

/// Define module's storage.
pub trait Storages {
    fn stateless_keys() -> &'static [&'static str];

    fn stateful_keys() -> &'static [&'static str];
}

/// Define backend key-value store.
pub trait KVStore {
    type Error;

    type Iter: core::iter::Iterator;

    fn set(&self, key: &[u8], value: Vec<u8>) -> core::result::Result<Option<Vec<u8>>, Self::Error>;

    fn get(&self, key: &[u8]) -> core::result::Result<Option<Vec<u8>>, Self::Error>;

    fn has(&self, key: &[u8]) -> core::result::Result<bool, Self::Error>;

    fn len(&self) -> usize;

    fn iter(&self) -> Self::Iter;
}

