#[cfg(feature = "http")]
mod http;
#[cfg(feature = "http")]
pub use http::HttpPostProvider;
#[cfg(feature = "http")]
pub use http::HttpGetProvider;

#[cfg(feature = "websocket")]
mod websocket;
#[cfg(feature = "websocket")]
pub use websocket::WsProvider;

use alloc::{boxed::Box, string::String};

use crate::error::Result;

#[async_trait::async_trait]
pub trait Provider {
    async fn request(&mut self, method: &str, params: &str) -> Result<Option<String>>;

    async fn receive(&mut self) -> Result<Option<String>>;
}
