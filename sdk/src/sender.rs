use crate::{error::Result, providers::Provider};
use abcf::module::ToBytes;
use alloc::string::String;
use serde_json::{json, Value};

pub async fn send_tx<P: Provider, T: ToBytes>(mut p: P, method: &str, tx: &T) -> Result<Option<Value>> {
    let tx_hex = String::from("0x") + &hex::encode(tx.to_bytes()?);
    let j = json!({ "tx": tx_hex });
    log::debug!("Send object is {}", j);
    p.request(&method, &j).await
}

pub async fn query_tx<P: Provider>(mut p: P, method: &str, hash: &str) -> Result<Option<Value>> {
    let hash = String::from("0x") + hash;
    let j = json!({"hash": hash});
    log::debug!("Send object is {}", j);
    p.request(&method, &j).await
}