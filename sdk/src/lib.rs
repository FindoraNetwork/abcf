pub mod rpc_sdk;

pub mod error;
pub mod event_sdk;

#[async_trait::async_trait]
pub trait Provider {

    async fn request(&self, params: String) -> error::Result<Option<String>>;

    async fn receive(&self) -> error::Result<Option<String>>;
}