use alloc::string::String;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Response<R> {
    /// JSON-RPC version
    pub jsonrpc: Option<String>,

    /// Identifier included in request
    pub id: Option<i64>,

    /// Results of request (if successful)
    pub result: Option<R>,

    /// Error message if unsuccessful
    pub error: Option<Value>,
}
