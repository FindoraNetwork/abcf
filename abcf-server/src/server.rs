use tokio::net::{ToSocketAddrs, TcpListener, TcpStream};
use crate::Result;
use crate::Codec;
use tendermint_proto::abci::{Request, Response};
use std::net::SocketAddr;

pub const DEFAULT_SERVER_READ_BUF_SIZE: usize = 1024 * 1024;

pub struct Server {
    listener: TcpListener,
}

fn mock_handle(req: Request) -> Response {
    log::info!("{:?}", req);
    Response::default()
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
                        addr, e
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
        Ok(Server {
            listener
        })
    }

    pub async fn run(self) -> Result<()> {
        loop {
            let (socket, addr) = self.listener.accept().await?;
            log::info!("new connect from {}", addr);
            tokio::spawn(conn_handle(socket, addr));
        }
    }
}

