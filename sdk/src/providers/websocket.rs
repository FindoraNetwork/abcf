use alloc::{boxed::Box, string::String};
use alloc::string::ToString;

use crate::error::Result;

use super::Provider;
use async_tungstenite::async_std::{connect_async, ConnectStream};
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};

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
    async fn request<Req, Resp>(&mut self, _method: &str, params: &Req) -> Result<Option<Resp>>
        where
            Req: Serialize + Sync + Send,
            Resp: for<'de> Deserialize<'de> + Send + Sync,
    {
        let url = "ws://localhost:26657/websocket";
        let p = serde_json::to_value(params)?;

        let (mut ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        ws_stream.send(Message::Text(p.to_string())).await?;
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
