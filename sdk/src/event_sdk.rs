use crate::error::Result;
use crate::Provider;

pub struct SubscribeProvider{}

#[async_trait::async_trait]
impl Provider for SubscribeProvider{
    async fn request(&self, params: String) -> Result<Option<String>> {
        Ok(None)
    }

    async fn receive(&self) -> Result<Option<String>> {
        Ok(None)
    }
}

pub struct Subscribe<P:Provider>{
    provider:P
}

impl<P:Provider> Subscribe<P> {
    fn subscribe(&self, query: String) -> Result<Option<Subscribe<P>>> {
        let resp = self.provider.request(query).await?;
        Ok(None)
    }
}