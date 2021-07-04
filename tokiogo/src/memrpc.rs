use tokio::runtime::Runtime;
use tokio::sync::mpsc::{self, Receiver, Sender};
// use std::marker::PhantomData;

#[async_trait::async_trait]
pub trait Server {
    async fn callable(&mut self, req: &[u8]) -> Vec<u8>;
}

pub struct MemServer<'a, S: Server> {
    requester: Receiver<&'a [u8]>,
    responser: Sender<Vec<u8>>,
    server: S,
}

pub struct MemClient<'a> {
    requester: Sender<&'a [u8]>,
    responser: Receiver<Vec<u8>>,
}

pub fn rpc<'a, S: Server>(buffer: usize, server: S) -> (MemClient<'a>, MemServer<'a, S>) {
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

impl<'a> MemClient<'a> {
    pub fn call(&mut self, req: &'a [u8]) -> Vec<u8> {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            self.requester.send(req).await.unwrap();
            self.responser.recv().await.unwrap()
        })
    }
}

impl<'a, S: Server> MemServer<'a, S> {
    pub fn start(&mut self) {
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            loop {
                let req = self.requester.recv().await.unwrap();
                let resp = self.server.callable(req).await;
                self.responser.send(resp).await.unwrap();
            }
        });
    }
}
