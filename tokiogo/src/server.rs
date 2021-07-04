use crate::Server;
use async_abci::abci::Application;
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
    async fn callable(&mut self, req: Vec<u8>) -> Vec<u8> {
        use abci::request::Value as Request;
        use abci::response::Value as Response;
        let req = abci::Request::decode(req.as_ref()).unwrap();
        // deal none
        let resp = match req.value.unwrap() {
            Request::Echo(r) => Response::Echo(self.abci_server.echo(r).await),
            Request::Flush(_) => Response::Flush(self.abci_server.flush().await),
            Request::Info(r) => Response::Info(self.abci_server.info(r).await),
            Request::SetOption(r) => Response::SetOption(self.abci_server.set_option(r).await),
            Request::InitChain(r) => Response::InitChain(self.abci_server.init_chain(r).await),
            Request::Query(r) => Response::Query(self.abci_server.query(r).await),
            Request::BeginBlock(r) => Response::BeginBlock(self.abci_server.begin_block(r).await),
            Request::CheckTx(r) => Response::CheckTx(self.abci_server.check_tx(r).await),
            Request::DeliverTx(r) => Response::DeliverTx(self.abci_server.deliver_tx(r).await),
            Request::EndBlock(r) => Response::EndBlock(self.abci_server.end_block(r).await),
            Request::Commit(_) => Response::Commit(self.abci_server.commit().await),
            Request::ListSnapshots(_) => {
                Response::ListSnapshots(self.abci_server.list_snapshots().await)
            }
            Request::OfferSnapshot(r) => {
                Response::OfferSnapshot(self.abci_server.offer_snapshot(r).await)
            }
            Request::LoadSnapshotChunk(r) => {
                Response::LoadSnapshotChunk(self.abci_server.load_snapshot_chunk(r).await)
            }
            Request::ApplySnapshotChunk(r) => {
                Response::ApplySnapshotChunk(self.abci_server.apply_snapshot_chunk(r).await)
            }
        };
        let r = abci::Response { value: Some(resp) };
        let mut r_bytes = Vec::new();
        r.encode(&mut r_bytes).unwrap();
        r_bytes
    }
}
