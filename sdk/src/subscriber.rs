use abcf::Event;
use alloc::string::String;

use crate::{error::Result, providers::Provider};
use core::marker::PhantomData;

pub struct Subscriber<P: Provider, E: Event> {
    provider: P,
    _e: PhantomData<E>,
}

impl<P: Provider, E: Event> Subscriber<P, E> {
    pub fn new(provider: P) -> Self {
        Self {
            provider,
            _e: PhantomData,
        }
    }

    pub async fn subscribe(&mut self, event: &str) -> Result<()> {
        let _ = self.provider.request("subscribe", event).await;
        Ok(())
    }

    pub async fn receive(&mut self) -> Result<Option<String>> {
        let data = self.provider.receive().await?;
        Ok(data)
    }

    pub async fn unsubcribe(&self) -> Result<()> {
        Ok(())
    }
}
