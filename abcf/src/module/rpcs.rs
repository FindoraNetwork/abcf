use alloc::boxed::Box;
use serde::Serialize;
use serde_json::Value;

// #[derive(Serialize)]
pub struct Response<'a, T: Serialize> {
    pub code: u32,
    pub message: &'a str,
    pub data: Option<T>,
}

#[async_trait::async_trait]
pub trait RPCs: Send + Sync {
    // async fn call(&mut self, ctx: &mut Context, method: &str, params: Value) -> Response<Value>;
    async fn call(&mut self, method: &str, params: Value) -> Response<'_, Value>;
}
