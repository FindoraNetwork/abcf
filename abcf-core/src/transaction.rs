use crate::Result;
use alloc::vec::Vec;

pub trait FromBytes {
    fn from_bytes(bytes: &[u8]) -> Result<Self>
    where
        Self: Sized;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Transaction: Default + FromBytes + ToBytes {}

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

impl Transaction for () {}
