use abcf::RPCResponse;
use abcf_sdk::error::{Error, Result};
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

pub async fn get_account<P: Provider>(
    p: P,
    param: GetAccountRequest,
) -> Result<RPCResponse<GetAccountResponse>> {
    let mut p = p;

    let data = serde_json::to_string(&param)?;
    let abci_query_req = endpoint::abci_query::Request {
        path: format!("rpc/{}/get_account", "adss"),
        data,
        height: Some("0".to_string()),
        prove: false,
    };

    let result: endpoint::abci_query::Response = p.request("abci_query", &abci_query_req).await?;

    if result.response.code == 0 {
        let res = serde_json::from_slice(&result.response.value)?;
        Ok(RPCResponse::new(res))
    } else {
        Err(Error::ReturnError(endpoint::Response::AbciQuery(result)))
    }
}

fn main() {}
