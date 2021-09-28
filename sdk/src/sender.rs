use crate::{
    error::{Error, Result},
    jsonrpc::Response,
    providers::Provider,
};
use abcf::module::ToBytes;
use serde_json::{json, Value};

pub async fn send_tx<P: Provider, T: ToBytes>(mut p: P, method: &str, tx: &T) -> Result<Value> {
    {
        let tx_hex = hex::encode(tx.to_bytes()?);
        let j = json!({ "tx": tx_hex });
        let resp: Response<Value> = p.request(&method, &j).await?;

        return if let Some(v) = resp.result {
            Ok(v)
        } else if let Some(e) = resp.error {
            Err(Error::RPCError(e))
        } else {
            Err(Error::NotImpl)
        };
    }
}
