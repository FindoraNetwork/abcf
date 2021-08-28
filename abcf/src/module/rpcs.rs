use crate::abci::Context;
use crate::Result;
use alloc::boxed::Box;
use core::fmt::Debug;
use serde::Serialize;
use serde_json::Value;

/// Response of RPC.
#[derive(Debug)]
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

impl<'a, T: Serialize> Response<'a, T> {
    pub fn new(t: T) -> Self {
        Self {
            code: 0,
            message: "success",
            data: Some(t)
        }
    }
}

/// Define module's RPC.
#[async_trait::async_trait]
pub trait RPCs: Send + Sync {
    async fn call(
        &mut self,
        ctx: &mut Context,
        method: &str,
        params: Value,
    ) -> Result<Option<Value>>;
}
