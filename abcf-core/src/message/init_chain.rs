use alloc::vec::Vec;
use alloc::string::String;

pub struct Timestamp {
    pub seconds: i64,
    pub nanos: i32,
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

pub struct RequestInitChain {
    pub time: Option<Timestamp>,
    pub chain_id: String,
    pub consensus_params: Option<ConsensusParams>,
    pub validators: Vec<ValidatorUpdate>,
    pub app_state_bytes: Vec<u8>,
    pub initial_height: i64,
}
