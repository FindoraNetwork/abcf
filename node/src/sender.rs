use abcf::entry::CacheSender;
use tm_protos::abci::{RequestBeginBlock, RequestDeliverTx, RequestEndBlock};

pub enum SenderValue {
    BeginValue(RequestBeginBlock),
    DeliverValue(RequestDeliverTx),
    EndValue(RequestEndBlock),
}

pub struct ChannelSender {
    pub sender: smol::channel::Sender<SenderValue>,
}

#[async_trait::async_trait]
impl CacheSender for ChannelSender {
    async fn begin_block(&self, req: RequestBeginBlock) {
        self.sender.send(SenderValue::BeginValue(req)).await.unwrap();
    }

    async fn deliver_tx(&self, req: RequestDeliverTx) {
        self.sender.send(SenderValue::DeliverValue(req)).await.unwrap();
    }

    async fn end_block(&self, req: RequestEndBlock) {
        self.sender.send(SenderValue::EndValue(req)).await.unwrap();
    }
}
