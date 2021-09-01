use crate::error::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcCallRequest {
    pub path: String,
    pub data: Option<Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RpcCallResponse {
    pub code: u64,
    pub message: String,
    pub data: Option<Value>,
}

pub struct AbciQueryRpcProvider{}

#[async_trait::async_trait]
impl crate::Provider for AbciQueryRpcProvider{
    async fn request(&self, params: String) -> Result<Option<String>> {
        let param = serde_json::from_str::<RpcCallRequest>(&params)?;
        return if let Some(resp) = rpc_call(param).await? {
            let s = serde_json::to_string(&resp)?;

            Ok(Some(s))
        } else {
            Ok(None)
        }
    }

    async fn receive(&self) -> Result<Option<String>> {
        Ok(None)
    }
}

async fn rpc_call(req: RpcCallRequest) -> Result<Option<RpcCallResponse>> {
    let mut data = String::from("");

    if let Some(v) = req.data {
        data = serde_json::to_string(&v)?;
    }

    let url = format!(
        "http://127.0.0.1:26657/abci_query?path={:?}&data={:?}",
        req.path, data
    );

    let resp_value = reqwest::get(url.as_str())
        .await?
        .json::<serde_json::Value>()
        .await?;

    let value = resp_value
        .as_object()
        .and_then(|map| map.get("result"))
        .and_then(|result_val| result_val.as_object())
        .and_then(|result_map| result_map.get("response"))
        .and_then(|response_val| response_val.as_object())
        .and_then(|response_map| response_map.get("value"))
        .and_then(|value| value.as_str())
        .and_then(|val| base64::decode(val).ok())
        .and_then(|bytes| serde_json::from_slice(bytes.as_slice()).ok());

    let resp = RpcCallResponse {
        code: 0,
        message: "success".to_string(),
        data: value,
    };

    Ok(Some(resp))
}
