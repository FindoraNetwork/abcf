use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Receiver, Sender};
// use std::marker::PhantomData;

#[async_trait::async_trait]
pub trait Server {
    async fn callable(&mut self, req: Vec<u8>) -> Vec<u8>;
}

pub struct MemServer<S: Server> {
    requester: Receiver<Vec<u8>>,
    responser: Sender<Vec<u8>>,
    server: S,
}

pub struct MemClient {
    requester: Sender<Vec<u8>>,
    responser: Receiver<Vec<u8>>,
}

pub fn rpc<S: Server>(buffer: usize, server: S) -> (MemClient, MemServer<S>) {
    // is &[u8]
    let (in_sender, in_receiver) = mpsc::channel(buffer);
    let (out_sender, out_recevier) = mpsc::channel(buffer);

    let server = MemServer {
        requester: in_receiver,
        responser: out_sender,
        server,
    };

    let client = MemClient {
        requester: in_sender,
        responser: out_recevier,
    };

    (client, server)
}

impl MemClient {
    pub fn call(&mut self, req: Vec<u8>) -> Vec<u8> {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            self.requester.send(req).await.unwrap();
            self.responser.recv().await.unwrap()
        })
    }
}

impl<S: Server> MemServer<S> {
    pub fn start(&mut self) {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let req = self.requester.recv().await.unwrap();
            let resp = self.server.callable(req).await;
            self.responser.send(resp).await.unwrap();
        });
    }
}
