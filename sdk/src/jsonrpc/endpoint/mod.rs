pub mod abci_query;
pub mod event;
pub mod subscribe;
pub mod tx;
pub mod unsubscribe;

pub mod net_info;
pub mod status;

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

pub fn deserialize_hex_bytes<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
    hex::decode(string).map_err(serde::de::Error::custom)
}

pub fn deserialize_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let bytes = Vec::<u8>::deserialize(deserializer)?;
    let string = String::from_utf8(bytes).map_err(serde::de::Error::custom)?;
    Ok(string)
}

pub fn deserialize_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
    string.parse::<i64>().map_err(serde::de::Error::custom)
}

pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let string = Option::<String>::deserialize(deserializer)?.unwrap_or_default();
    string.parse::<u64>().map_err(serde::de::Error::custom)
}
