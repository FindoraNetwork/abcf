#![feature(generic_associated_types)]

use abcf::bs3::{
    merkle::append_only::AppendOnlyMerkle,
    model::{Map, Value},
};
use abcf::RPCResponse;
use abcf_macros::{module, rpcs};
use serde::{Deserialize, Serialize};

#[module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct RpcTest {
    pub inner: u32,
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

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
        _ctx: &mut abcf::RPCContext<'_, Self>,
        params: GetAccountRequest,
    ) -> RPCResponse<GetAccountResponse> {
        let resp = GetAccountResponse {
            name: "jack".to_string(),
            code: params.code,
        };
        RPCResponse::new(resp)
    }

    pub async fn get_account1(
        &mut self,
        _ctx: &mut abcf::RPCContext<'_, Self>,
        params: GetAccountRequest,
    ) -> RPCResponse<GetAccountResponse> {
        let resp = GetAccountResponse {
            name: "jack".to_string(),
            code: params.code,
        };
        RPCResponse::new(resp)
    }
}

pub mod call_rpc {
    include!(concat!(env!("OUT_DIR"), "/rpctest.rs"));
}

#[tokio::main]
async fn main() {
    // TODO: use node as example.
    // let _es = EmptyStruct {};
    //
    // let mut rt = RpcTest {};
    //
    // let mut context = Context {
    //     event: None,
    //     storage: StorageContext {},
    //     calls: CallContext { name_index: (), calls: () }
    // };
    //
    // let params = GetAccountRequest { code: 99 };
    // let params = serde_json::to_value(params).unwrap();
    //
    // let resp = rt.call(&mut context, "get_account", params).await.unwrap();
    //
    // let resp = serde_json::from_value::<GetAccountResponse>(resp.unwrap()).unwrap();
    //
    // assert_eq!(resp.name, "jack");
    // assert_eq!(resp.code, 99);
}
