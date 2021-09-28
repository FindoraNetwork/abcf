pub mod abci_query;
pub mod subscribe;
pub mod unsubscribe;

#[derive(Debug)]
pub enum Response {
    AbciQuery(abci_query::Response),
}

use alloc::{string::String, vec::Vec};
use hex::ToHex;
use serde::{Deserialize, Deserializer, Serializer};

pub fn serialize<S, T>(data: T, serializer: S) -> core::result::Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToHex,
{
    let s = data.encode_hex::<String>();
    let result = String::from("0x") + &s;
    serializer.serialize_str(&result)
}

pub fn deserialize_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
    base64::decode(&string).map_err(serde::de::Error::custom)
}

pub fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes = Vec::<u8>::deserialize(deserializer)?;
    let string = String::from_utf8(bytes).map_err(serde::de::Error::custom)?;
    Ok(string)
}
