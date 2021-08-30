use abcf::abci::{Context, StorageContext};
use abcf::{RPCResponse, RPCs};
use abcf_macros::rpcs;
use serde::{Deserialize, Serialize};

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

pub struct EmptyStruct {}

#[rpcs]
impl EmptyStruct {}

#[rpcs]
impl RpcTest {
    pub async fn get_account(
        &mut self,
        _ctx: &mut Context<'_>,
        params: GetAccountRequest,
    ) -> RPCResponse<'_, GetAccountResponse> {
        let resp = GetAccountResponse {
            name: "jack".to_string(),
            code: params.code,
        };
        RPCResponse::new(resp)
    }
}

#[tokio::main]
async fn main() {
    let _es = EmptyStruct {};

    let mut rt = RpcTest {};

    let mut context = Context {
        event: None,
        storage: StorageContext {},
    };

    let params = GetAccountRequest { code: 99 };
    let params = serde_json::to_value(params).unwrap();

    let resp = rt.call(&mut context, "get_account", params).await.unwrap();

    let resp = serde_json::from_value::<GetAccountResponse>(resp.unwrap()).unwrap();

    assert_eq!(resp.name, "jack");
    assert_eq!(resp.code, 99);
}
