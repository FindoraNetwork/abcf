use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(serialize_with = "super::serialize")]
    pub path: String,

    #[serde(serialize_with = "super::serialize")]
    pub data: Vec<u8>,

    pub height: Option<String>,

    pub prove: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub response: RealResponse,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RealResponse {
    pub code: u32,

    pub log: String,

    pub info: String,

    pub index: String,

    #[serde(deserialize_with = "super::deserialize_bytes")]
    pub key: Vec<u8>,

    #[serde(deserialize_with = "super::deserialize_bytes")]
    pub value: Vec<u8>,

    pub height: String,

    pub codespace: String,
}
