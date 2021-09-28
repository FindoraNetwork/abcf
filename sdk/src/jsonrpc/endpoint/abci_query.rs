use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Request {
    #[serde(serialize_with = "hex::serialize")]
    pub path: String,

    pub data: String,

    pub height: Option<String>,

    #[serde(default)]
    pub prove: bool,
}
