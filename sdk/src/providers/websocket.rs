use alloc::{boxed::Box, string::String};

use crate::error::Result;

use super::Provider;
use alloc::string::ToString;
use async_tungstenite::async_std::{connect_async, ConnectStream};
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use futures::{SinkExt, StreamExt};

pub struct WsProvider {
    receiver: Option<WebSocketStream<ConnectStream>>,
}

impl WsProvider {
    pub fn new() -> Self {
        Self { receiver: None }
    }
}

#[async_trait::async_trait]
impl Provider for WsProvider {
    async fn request(&mut self, _method: &str, params: &str) -> Result<Option<String>> {
        let url = "ws://127.0.0.1:26657/websocket";

        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        ws_stream.send(Message::Text(params.to_string())).await?;
        ws_stream.next().await.ok_or("didn't receive anything")??;

        self.receiver = Some(ws_stream);
        Ok(None)
    }

    async fn receive(&mut self) -> Result<Option<String>> {
        let msg = self.receiver.as_mut().unwrap().next().await.unwrap()?;
        return match msg {
            Message::Text(text) => Ok(Some(text)),
            Message::Binary(bytes) => {
                let str = String::from_utf8(bytes)?;
                Ok(Some(str))
            }
            _ => Ok(None),
        };
    }
}
