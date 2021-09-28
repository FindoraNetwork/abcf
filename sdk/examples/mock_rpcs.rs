use abcf_sdk::error::*;
use abcf_sdk::jsonrpc::{endpoint, Request};
use abcf_sdk::providers::{HttpGetProvider, Provider};
use serde_json::{json, Value};
use tokio::runtime::Runtime;

pub async fn get_account<P: Provider>(param: Value, mut p: P) -> Result<Option<Value>> {
    let data = param.as_str().unwrap().to_string();
    let abci_query_req = endpoint::abci_query::Request {
        path: "rpc/mock/get_account".to_string(),
        data,
        height: Some("0".to_string()),
        prove: false,
    };

    let req = Request::new_to_str("abci_query", abci_query_req);

    let resp = p.request("abci_query", req.as_str()).await?;

    return if let Some(val) = resp {
        let json = serde_json::from_str::<Value>(&val)?;
        Ok(Some(json))
    } else {
        Ok(None)
    };
}

fn main() {
    let rt = Runtime::new().unwrap();
    let json = json!({"code":19});
    let str = serde_json::to_string(&json).unwrap();
    let req_hex = hex::encode(str.as_bytes());
    let req = Value::String(req_hex);

    let provider = HttpGetProvider {};

    rt.block_on(async {
        let r = get_account(req, provider).await;
        println!("{:#?}", r);
    });
}
