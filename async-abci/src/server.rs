use crate::{Codec, Error, Result};
use std::net::SocketAddr;
use std::sync::Arc;
use tm_protos::abci::{request::Value, response, Request, Response};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use tokio::sync::Mutex;
use tm_abci::Application;

pub const DEFAULT_SERVER_READ_BUF_SIZE: usize = 1024 * 1024;

async fn conn_handle<A>(socket: TcpStream, addr: SocketAddr, app: Arc<Mutex<A>>)
where
    A: Application,
{
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

        let mut app = app.lock().await;

        log::debug!("Recv packet {:?}", request);
        let response = dispatch(&mut *app, request).await;
        log::debug!("Return packet {:?}", response);

        if let Err(e) = codec.send(response).await {
            log::error!("Failed sending response to client {}: {:?}", addr, e);
            return;
        }
    }
}

pub async fn dispatch<A>(app: &mut A, request: Request) -> Response
where
    A: Application,
{
    Response {
        value: Some(match request.value.unwrap() {
            Value::Echo(req) => response::Value::Echo(app.echo(req).await),
            Value::Flush(_) => response::Value::Flush(app.flush().await),
            Value::Info(req) => response::Value::Info(app.info(req).await),
            Value::SetOption(req) => response::Value::SetOption(app.set_option(req).await),
            Value::InitChain(req) => response::Value::InitChain(app.init_chain(req).await),
            Value::Query(req) => response::Value::Query(app.query(req).await),
            Value::BeginBlock(req) => response::Value::BeginBlock(app.begin_block(req).await),
            Value::CheckTx(req) => response::Value::CheckTx(app.check_tx(req).await),
            Value::DeliverTx(req) => response::Value::DeliverTx(app.deliver_tx(req).await),
            Value::EndBlock(req) => response::Value::EndBlock(app.end_block(req).await),
            Value::Commit(_) => response::Value::Commit(app.commit().await),
            Value::ListSnapshots(_) => response::Value::ListSnapshots(app.list_snapshots().await),
            Value::OfferSnapshot(req) => {
                response::Value::OfferSnapshot(app.offer_snapshot(req).await)
            }
            Value::LoadSnapshotChunk(req) => {
                response::Value::LoadSnapshotChunk(app.load_snapshot_chunk(req).await)
            }
            Value::ApplySnapshotChunk(req) => {
                response::Value::ApplySnapshotChunk(app.apply_snapshot_chunk(req).await)
            }
        }),
    }
}

pub struct Server<A: Application> {
    listener: Option<TcpListener>,
    app: Arc<Mutex<A>>,
}

impl<A: Application + 'static> Server<A> {
    pub fn new(app: A) -> Self {
        Server {
            listener: None,
            app: Arc::new(Mutex::new(app)),
        }
    }

    pub async fn bind<Addr: ToSocketAddrs>(mut self, addr: Addr) -> Result<Self> {
        let listener = TcpListener::bind(addr).await?;
        self.listener = Some(listener);
        Ok(self)
    }

    pub async fn run(self) -> Result<()> {
        if self.listener.is_none() {
            return Err(Error::ServerNotBinding);
        }
        let listener = self.listener.unwrap();
        loop {
            let (socket, addr) = listener.accept().await?;
            log::info!("new connect from {}", addr);
            tokio::spawn(conn_handle(socket, addr, self.app.clone()));
        }
    }
}
