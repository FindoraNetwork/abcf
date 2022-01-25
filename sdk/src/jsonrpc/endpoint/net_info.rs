use alloc::{string::String, vec::Vec};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PeerInfo {
    pub node_id: String,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub listening: bool,
    pub listeners: Vec<String>,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub n_peers: u64,
    pub peers: Option<Vec<PeerInfo>>,
}
