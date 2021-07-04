use alloc::string::String;
use alloc::vec::Vec;

pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
}

pub struct BlockParams {
    pub max_bytes: i64,
    pub max_gas: i64,
}

pub struct Duration {
    pub seconds: i64,
    pub nanos: i32,
}

pub struct EvidenceParams {
    pub max_age_num_blocks: i64,
    pub max_age_duration: Option<Duration>,
    pub max_bytes: i64,
}

pub struct ValidatorParams {
    pub pub_key_types: Vec<String>,
}

pub struct VersionParams {
    pub app_version: u64,
}

pub struct ConsensusParams {
    pub block: Option<BlockParams>,
    pub evidence: Option<EvidenceParams>,
    pub validator: Option<ValidatorParams>,
    pub version: Option<VersionParams>,
}

pub enum PublicKey {
    Ed25519(Vec<u8>),
    Secp256k1(Vec<u8>),
}

pub struct ValidatorUpdate {
    pub pub_key: Option<PublicKey>,
    pub power: i64,
}

pub struct Request {
    pub time: Option<Timestamp>,
    pub chain_id: String,
    pub consensus_params: Option<ConsensusParams>,
    pub validators: Vec<ValidatorUpdate>,
    pub app_state_bytes: Vec<u8>,
    pub initial_height: i64,
}
