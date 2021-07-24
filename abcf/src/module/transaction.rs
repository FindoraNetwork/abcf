use crate::Result;
use alloc::vec::Vec;

/// Convert from bytes.
pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

/// Convert to bytes.
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

/// Transaction trait.
pub trait Transaction: Default + FromBytes + ToBytes + Send {}

impl FromBytes for () {
    fn from_bytes(_bytes: &[u8]) -> Result<Self> {
        Ok(())
    }
}

impl ToBytes for () {
    fn to_bytes(&self) -> Vec<u8> {
        Vec::new()
    }
}
