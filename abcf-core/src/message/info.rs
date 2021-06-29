#[derive(Default)]
pub struct Request {
    pub version: String,
    pub block_version: u64,
    pub p2p_version: u64,
    pub abci_version: String,
}

#[derive(Default)]
pub struct Response {
    pub data: String,
    pub version: String,
    pub app_version: u64,
    pub last_block_height: i64,
    pub last_block_app_hash: Vec<u8>,
}
