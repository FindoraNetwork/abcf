use abcf_sdk::error::Result;
use abcf_sdk::jsonrpc::endpoint;
use abcf_sdk::providers::Provider;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountRequest {
    code: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountResponse {
    name: String,
    code: u8,
}

pub async fn get_account<P: Provider>(p: P, param: GetAccountRequest) -> Result<serde_json::Value> {
    let mut p = p;

    let data = serde_json::to_string(&param)?;
    let abci_query_req = endpoint::abci_query::Request {
        path: format!("rpc/{}/get_account", "asd"),
        data,
        height: Some("0".to_string()),
        prove: false,
    };

    p.request("abci_query", &abci_query_req).await
}

fn main() {}
