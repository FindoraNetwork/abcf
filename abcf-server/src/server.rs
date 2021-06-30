use crate::Codec;
use crate::Result;
use std::net::SocketAddr;
use tendermint_proto::abci::{Request, Response};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

pub const DEFAULT_SERVER_READ_BUF_SIZE: usize = 1024 * 1024;

pub struct Server {
    listener: TcpListener,
}

fn mock_handle(req: Request) -> Response {
    log::info!("{:?}", req);
    match req.value {
        Some(r) => Response { value: mock::default_reqresp(r) },
        None => Response { value: None },
    }
}

async fn conn_handle(socket: TcpStream, addr: SocketAddr) {
    let mut codec = Codec::new(socket, DEFAULT_SERVER_READ_BUF_SIZE);

    loop {
        let request = match codec.next().await {
            Some(result) => match result {
                Ok(r) => r,
                Err(e) => {
                    log::info!(
                        "Failed to read incoming request from client {}: {:?}",
                        addr,
                        e
                    );
                    return;
                }
            },
            None => {
                log::info!("Client {} terminated stream", addr);
                return;
            }
        };

        let response = mock_handle(request);

        if let Err(e) = codec.send(response).await {
            log::error!("Failed sending response to client {}: {:?}", addr, e);
            return;
        }
    }
}

impl Server {
    pub async fn bind<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        log::info!("listen at");
        Ok(Server { listener })
    }

    pub async fn run(self) -> Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;
            log::info!("new connect from {}", addr);
            tokio::spawn(conn_handle(socket, addr));
        }
    }
}

mod mock {
    use tendermint_proto::abci::{request, response};
    use tendermint_proto::abci::{
        ResponseApplySnapshotChunk, ResponseBeginBlock, ResponseCheckTx, ResponseCommit,
        ResponseDeliverTx, ResponseEcho, ResponseEndBlock, ResponseFlush, ResponseInfo,
        ResponseInitChain, ResponseListSnapshots, ResponseLoadSnapshotChunk, ResponseOfferSnapshot,
        ResponseQuery, ResponseSetOption,
    };

    pub fn default_reqresp(req: request::Value) -> Option<response::Value> {
        match req {
            request::Value::Echo(_) => Some(response::Value::Echo(ResponseEcho::default())),
            request::Value::Flush(_) => Some(response::Value::Flush(ResponseFlush::default())),
            request::Value::Info(_) => Some(response::Value::Info(ResponseInfo::default())),
            request::Value::SetOption(_) => {
                Some(response::Value::SetOption(ResponseSetOption::default()))
            }
            request::Value::InitChain(_) => {
                Some(response::Value::InitChain(ResponseInitChain::default()))
            }
            request::Value::Query(_) => Some(response::Value::Query(ResponseQuery::default())),
            request::Value::BeginBlock(_) => {
                Some(response::Value::BeginBlock(ResponseBeginBlock::default()))
            }
            request::Value::CheckTx(_) => {
                Some(response::Value::CheckTx(ResponseCheckTx::default()))
            }
            request::Value::DeliverTx(_) => {
                Some(response::Value::DeliverTx(ResponseDeliverTx::default()))
            }
            request::Value::EndBlock(_) => {
                Some(response::Value::EndBlock(ResponseEndBlock::default()))
            }
            request::Value::Commit(_) => Some(response::Value::Commit(ResponseCommit::default())),
            request::Value::ListSnapshots(_) => Some(response::Value::ListSnapshots(
                ResponseListSnapshots::default(),
            )),
            request::Value::OfferSnapshot(_) => Some(response::Value::OfferSnapshot(
                ResponseOfferSnapshot::default(),
            )),
            request::Value::LoadSnapshotChunk(_) => Some(response::Value::LoadSnapshotChunk(
                ResponseLoadSnapshotChunk::default(),
            )),
            request::Value::ApplySnapshotChunk(_) => Some(response::Value::ApplySnapshotChunk(
                ResponseApplySnapshotChunk::default(),
            )),
        }
    }
}
