use crate::abci::Context;
use alloc::boxed::Box;
use serde::Serialize;
use serde_json::Value;

/// Response of RPC.
pub struct Response<'a, T: Serialize> {
    pub code: u32,
    pub message: &'a str,
    pub data: Option<T>,
}

impl<'a, T: Serialize> Default for Response<'a, T> {
    fn default() -> Self {
        Self {
            code: 0,
            message: "success",
            data: None,
        }
    }
}

/// Define module's RPC.
#[async_trait::async_trait]
pub trait RPCs: Send + Sync {
    async fn call(&mut self, ctx: &mut Context, method: &str, params: Value)
        -> Response<'_, Value>;
}
