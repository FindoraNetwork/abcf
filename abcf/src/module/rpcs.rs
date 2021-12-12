use crate::manager::{ModuleStorage, ModuleStorageDependence};
use crate::{Error, RPCContext, Result};
use alloc::boxed::Box;
use alloc::string::String;
use core::fmt::Debug;
use serde::Serialize;
use serde_json::Value;

/// Response of RPC.
#[derive(Debug)]
pub struct Response<T: Serialize> {
    pub code: u32,
    pub message: String,
    pub data: Option<T>,
}

impl<T: Serialize> Default for Response<T> {
    fn default() -> Self {
        Self {
            code: 0,
            message: String::from("success"),
            data: None,
        }
    }
}

impl<T: Serialize> From<Error> for Response<T> {
    fn from(e: Error) -> Self {
        Self {
            code: e.code(),
            message: e.message(),
            data: None,
        }
    }
}

impl<T: Serialize> Response<T> {
    pub fn new(t: T) -> Self {
        Self {
            code: 0,
            message: String::from("success"),
            data: Some(t),
        }
    }
}

/// Define module's RPC.
#[async_trait::async_trait]
pub trait RPCs: Send + Sync {
    async fn call<'a>(
        &mut self,
        context: RPCContext<'a, Self>,
        method: &str,
        params: Value,
    ) -> Result<Option<Value>>
    where
        Self: ModuleStorageDependence<'a> + ModuleStorage;
}
