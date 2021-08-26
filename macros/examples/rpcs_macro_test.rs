use abcf::abci::{Context, StorageContext};
use abcf::{Error, RPCResponse, RPCs, Result};
use abcf_macros::rpcs;
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use tm_protos::abci::Response;

pub struct RpcTest {}

#[derive(Deserialize, Serialize, Debug)]
pub struct GetAccountRequest {
    code: u8,
}

#[derive(Serialize)]
pub struct GetAccountResponse {}

#[rpcs]
impl RpcTest {
    pub async fn get_account(
        &mut self,
        ctx: &mut Context<'_>,
        params: GetAccountRequest,
    ) -> Result<Value> {
        Ok(Value::Number(Number::from(params.code)))
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

    let resp = rt.call(&mut context, "get_account", params).await;

    assert_eq!(resp.data, Some(Value::Number(Number::from(99))));
}
