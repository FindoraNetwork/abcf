use abcf::entry::CacheSender;
use tm_protos::abci::{RequestBeginBlock, RequestDeliverTx, RequestEndBlock};

pub enum SenderValue {
    BeginBlock(RequestBeginBlock),
    DeliverTx(RequestDeliverTx),
    EndBlock(RequestEndBlock),
}

pub struct ChannelSender {
    pub sender: smol::channel::Sender<SenderValue>,
}

#[async_trait::async_trait]
impl CacheSender for ChannelSender {
    async fn begin_block(&self, req: RequestBeginBlock) {
        self.sender.send(SenderValue::BeginBlock(req)).await.unwrap();
    }

    async fn deliver_tx(&self, req: RequestDeliverTx) {
        self.sender.send(SenderValue::DeliverTx(req)).await.unwrap();
    }

    async fn end_block(&self, req: RequestEndBlock) {
        self.sender.send(SenderValue::EndBlock(req)).await.unwrap();
    }
}
