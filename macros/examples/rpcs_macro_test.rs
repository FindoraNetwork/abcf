use abcf::abci::{Context, StorageContext};
use abcf::{Error, RPCResponse, RPCs, Result};
use abcf_macros::rpcs;
use serde::{Deserialize, Serialize};
use tm_protos::abci::Response;

pub struct RpcTest {}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountRequest {
    code: u8,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountResponse {
    name: String,
    code: u8,
}

#[rpcs]
impl RpcTest {
    pub async fn get_account(
        &mut self,
        ctx: &mut Context<'_>,
        params: GetAccountRequest,
    ) -> Result<GetAccountResponse> {
        let resp = GetAccountResponse {
            name: "jack".to_string(),
            code: params.code,
        };
        Ok(resp)
    }
}

#[tokio::main]
async fn main() {
    let mut rt = RpcTest {};

    let mut context = Context {
        event: None,
        storage: StorageContext {},
    };

    let params = GetAccountRequest { code: 99 };
    let params = serde_json::to_value(params).unwrap();

    let resp = rt.call(&mut context, "get_account", params).await.unwrap();

    let resp = serde_json::from_value::<GetAccountResponse>(resp.data.unwrap()).unwrap();

    assert_eq!(resp.name, "jack");
    assert_eq!(resp.code, 99);
}
