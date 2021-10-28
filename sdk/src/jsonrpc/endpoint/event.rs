use alloc::{string::String, vec::Vec};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventAttribute {
    #[serde(deserialize_with = "super::deserialize_bytes")]
    pub key: Vec<u8>,

    #[serde(deserialize_with = "super::deserialize_bytes")]
    pub value: Vec<u8>,

    pub index: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    #[serde(rename = "type")]
    pub ty: String,

    pub attributes: Vec<EventAttribute>,
}
