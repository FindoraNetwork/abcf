use alloc::string::String;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct Request<R> {
    pub jsonrpc: String,

    pub id: i64,

    pub method: String,

    pub params: R,
}

impl<R: Serialize> Request<R> {
    pub fn new(method: &str, params: R) -> Self {
        let id: i64 = rand::thread_rng().gen();
        Self {
            jsonrpc: String::from("2.0"),
            id,
            method: String::from(method),
            params,
        }
    }

    pub fn new_to_str(method: &str, params: R) -> String {
        let req = Request::new(method, params);
        let json = serde_json::to_string(&req).unwrap();
        json
    }

    pub fn new_to_value(method: &str, params: R) -> Value {
        let req = Request::new(method, params);
        let value = serde_json::to_value(req).unwrap();
        value
    }
}
