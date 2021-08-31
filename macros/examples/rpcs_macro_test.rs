use abcf::abci::{CallContext, Context, StorageContext};
use abcf::{RPCResponse, RPCs};
use abcf_macros::{module, rpcs};
use abcf_sdk::rpc_sdk::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

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
#[module(mock)]
impl EmptyStruct {}

#[rpcs]
#[module(mock)]
impl RpcTest {
    pub async fn get_account(
        &mut self,
        _ctx: &mut Context<'_>,
        params: Value,
    ) -> RPCResponse<'_, GetAccountResponse> {
        let req = serde_json::from_value::<GetAccountRequest>(params).unwrap();
        let resp = GetAccountResponse {
            name: "jack".to_string(),
            code: req.code,
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
        calls: CallContext {
            name_index: &Default::default(),
            calls: &mut vec![],
        },
    };

    let params = GetAccountRequest { code: 99 };
    let params = serde_json::to_value(params).unwrap();

    let resp = rt.call(&mut context, "get_account", params).await.unwrap();

    let resp = serde_json::from_value::<GetAccountResponse>(resp.unwrap()).unwrap();

    assert_eq!(resp.name, "jack");
    assert_eq!(resp.code, 99);
}
