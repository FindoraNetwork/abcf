use crate::Server;
use async_abci::dispatch;
use prost::Message;
use tendermint_proto::abci;

pub type ABCIApplication = ();

pub struct ABCIMemServer {
    abci_server: ABCIApplication,
}

impl ABCIMemServer {
    pub fn new(server: ABCIApplication) -> Self {
        Self {
            abci_server: server,
        }
    }
}

#[async_trait::async_trait]
impl Server for ABCIMemServer {
    async fn callable(&mut self, req: &[u8]) -> Vec<u8> {
        let req = abci::Request::decode(req.as_ref()).unwrap();
        log::debug!("{:?}", req);
        let resp = dispatch(&mut self.abci_server, req).await;
        let mut r_bytes = Vec::new();
        resp.encode(&mut r_bytes).unwrap();
        r_bytes
    }
}
