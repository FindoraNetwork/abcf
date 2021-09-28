use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(serialize_with = "hex::serialize")]
    pub path: String,

    #[serde(serialize_with = "hex::serialize")]
    pub data: String,

    pub height: Option<String>,

    pub prove: bool,
}
