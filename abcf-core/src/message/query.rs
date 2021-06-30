use alloc::string::String;
use alloc::vec::Vec;

#[derive(Default)]
pub struct Request {
    pub data: Vec<u8>,
    pub path: String,
    pub height: i64,
    pub prove: bool,
}

pub struct ProofOp {
    pub t: String,
    pub key: Vec<u8>,
    pub data: Vec<u8>,
}

pub struct ProofOps {
    pub ops: Vec<ProofOp>,
}

#[derive(Default)]
pub struct Response {
    pub code: u32,
    pub log: String,
    pub info: String,
    pub index: i64,
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub proof_ops: Option<ProofOps>,
    pub height: i64,
    pub codespace: String,
}
