use alloc::{string::String, vec::Vec};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ProtocolVersion {
    pub p2p: String,
    pub block: String,
    pub app: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Other {
    pub tx_index: String,
    pub rpc_address: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeInfo {
    pub protocol_version: ProtocolVersion,
    #[serde(deserialize_with = "super::deserialize_bytes")]
    pub id: Vec<u8>,
    pub listen_addr: String,
    pub network: String,
    pub version: String,
    pub channels: String,
    pub moniker: String,
    pub other: Other,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncInfo {
    #[serde(deserialize_with = "super::deserialize_hex_bytes")]
    pub latest_block_hash: Vec<u8>,
    #[serde(deserialize_with = "super::deserialize_hex_bytes")]
    pub latest_app_hash: Vec<u8>,
    #[serde(deserialize_with = "super::deserialize_i64")]
    pub latest_block_height: i64,
    pub latest_block_time: String,
    #[serde(deserialize_with = "super::deserialize_hex_bytes")]
    pub earliest_block_hash: Vec<u8>,
    #[serde(deserialize_with = "super::deserialize_hex_bytes")]
    pub earliest_app_hash: Vec<u8>,
    #[serde(deserialize_with = "super::deserialize_i64")]
    pub earliest_block_height: i64,
    pub earliest_block_time: String,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub max_peer_block_height: u64,
    pub catching_up: bool,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub total_synced_time: u64,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub remaining_time: u64,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub total_snapshots: u64,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub chunk_process_avg_time: u64,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub snapshot_height: u64,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub snapshot_chunks_count: u64,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub snapshot_chunks_total: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicKey {
    pub r#type: String,
    #[serde(deserialize_with = "super::deserialize_bytes")]
    pub value: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ValidatorInfo {
    #[serde(deserialize_with = "super::deserialize_hex_bytes")]
    pub address: Vec<u8>,
    pub pub_key: PublicKey,
    #[serde(deserialize_with = "super::deserialize_u64")]
    pub voting_power: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub node_info: NodeInfo,
    pub sync_info: SyncInfo,
    pub validator_info: ValidatorInfo,
}
