use alloc::vec::Vec;
use core::fmt::Debug;
use serde::{Deserialize, Serialize};

pub trait Value: Clone + Debug + Default + Serialize + for<'de> Deserialize<'de> {}

/// Define module's storage.
pub trait Storage {}

impl Storage for () {}

/// Define basic key-value store.
pub trait Map {
    type Error;

    fn set(&self, key: &[u8], value: Vec<u8>)
        -> core::result::Result<Option<Vec<u8>>, Self::Error>;

    fn get(&self, key: &[u8]) -> core::result::Result<Option<Vec<u8>>, Self::Error>;

    fn has(&self, key: &[u8]) -> core::result::Result<bool, Self::Error>;
}

/// Define backend key-value store.
pub trait KVStore: Map {
    type Iter: core::iter::Iterator;

    type TransactionMap: Map<Error = Self::Error>;

    fn transaction(&self, tx: Self::TransactionMap) -> Result<(), Self::Error>;

    fn len(&self) -> usize;

    fn iter(&self) -> Self::Iter;
}
