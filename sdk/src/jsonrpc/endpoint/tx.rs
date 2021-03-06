use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    #[serde(serialize_with = "super::serialize")]
    pub hash: Vec<u8>,

    pub prove: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    #[serde(deserialize_with = "super::deserialize_hex_bytes")]
    pub hash: Vec<u8>,

    #[serde(deserialize_with = "super::deserialize_i64")]
    pub height: i64,

    pub index: i64,

    #[serde(deserialize_with = "super::deserialize_bytes")]
    pub tx: Vec<u8>,
    pub tx_result: ResultResponse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultResponse {
    pub code: i64,

    pub data: String,

    pub log: String,

    pub codespace: String,

    pub hash: String,
}
