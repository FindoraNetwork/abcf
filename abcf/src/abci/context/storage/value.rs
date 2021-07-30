use core::{fmt::Debug, marker::PhantomData};

use alloc::vec::Vec;
use digest::Digest;
use generic_array::typenum;
use sparse_merkle_tree::{traits::Value, H256};

pub struct StoragedValue<H: Digest<OutputSize = typenum::U32>> {
    value: Vec<u8>,
    marker: PhantomData<H>,
}

impl<H: Digest<OutputSize = typenum::U32>> Default for StoragedValue<H> {
    fn default() -> Self {
        Self {
            value: Vec::new(),
            marker: PhantomData,
        }
    }
}

impl<H: Digest<OutputSize = typenum::U32>> Debug for StoragedValue<H> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.value.fmt(f)
    }
}

impl<H: Digest<OutputSize = typenum::U32>> StoragedValue<H> {
    pub fn new(value: Vec<u8>) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }
}

impl<H: Digest<OutputSize = typenum::U32>> Value for StoragedValue<H> {
    fn to_h256(&self) -> H256 {
        if self.value.is_empty() {
            return H256::zero();
        }

        let mut hasher = H::new();
        hasher.update(&self.value);
        let result = hasher.finalize();
        let r: [u8; 32] = result.into();
        r.into()
    }

    fn zero() -> Self {
        Self::default()
    }
}
