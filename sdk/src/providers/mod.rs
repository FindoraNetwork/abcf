#[cfg(feature = "http")]
mod http;
#[cfg(feature = "http")]
pub use http::HttpGetProvider;
#[cfg(feature = "http")]
pub use http::HttpPostProvider;

#[cfg(feature = "websocket")]
mod websocket;
use serde::Deserialize;
use serde::Serialize;
#[cfg(feature = "websocket")]
pub use websocket::WsProvider;

use alloc::{boxed::Box, string::String};

use crate::error::Result;

#[async_trait::async_trait]
pub trait Provider {
    async fn request<Req, Resp>(&mut self, method: &str, params: &Req) -> Result<Resp>
    where
        Req: Serialize + Send + Sync,
        Resp: for<'de> Deserialize<'de> + Send + Sync;

    async fn receive(&mut self) -> Result<Option<String>>;
}
