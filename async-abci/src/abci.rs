pub use tendermint_proto::abci::{
    RequestApplySnapshotChunk, RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEcho,
    RequestEndBlock, RequestInfo, RequestInitChain, RequestLoadSnapshotChunk, RequestOfferSnapshot,
    RequestQuery, RequestSetOption, ResponseApplySnapshotChunk, ResponseBeginBlock,
    ResponseCheckTx, ResponseCommit, ResponseDeliverTx, ResponseEcho, ResponseEndBlock,
    ResponseFlush, ResponseInfo, ResponseInitChain, ResponseListSnapshots,
    ResponseLoadSnapshotChunk, ResponseOfferSnapshot, ResponseQuery, ResponseSetOption,
};

#[async_trait::async_trait]
pub trait Application: Send {
    async fn echo(&mut self, request: RequestEcho) -> ResponseEcho {
        ResponseEcho {
            message: request.message,
        }
    }

    async fn info(&mut self, _request: RequestInfo) -> ResponseInfo {
        Default::default()
    }

    async fn init_chain(&mut self, _request: RequestInitChain) -> ResponseInitChain {
        Default::default()
    }

    async fn query(&mut self, _request: RequestQuery) -> ResponseQuery {
        Default::default()
    }

    async fn check_tx(&mut self, _request: RequestCheckTx) -> ResponseCheckTx {
        Default::default()
    }

    async fn begin_block(&mut self, _request: RequestBeginBlock) -> ResponseBeginBlock {
        Default::default()
    }

    async fn deliver_tx(&mut self, _request: RequestDeliverTx) -> ResponseDeliverTx {
        Default::default()
    }

    async fn end_block(&mut self, _request: RequestEndBlock) -> ResponseEndBlock {
        Default::default()
    }

    async fn flush(&mut self) -> ResponseFlush {
        ResponseFlush {}
    }

    async fn commit(&mut self) -> ResponseCommit {
        Default::default()
    }

    async fn set_option(&mut self, _request: RequestSetOption) -> ResponseSetOption {
        Default::default()
    }

    async fn list_snapshots(&mut self) -> ResponseListSnapshots {
        Default::default()
    }

    async fn offer_snapshot(&mut self, _request: RequestOfferSnapshot) -> ResponseOfferSnapshot {
        Default::default()
    }

    async fn load_snapshot_chunk(
        &mut self,
        _request: RequestLoadSnapshotChunk,
    ) -> ResponseLoadSnapshotChunk {
        Default::default()
    }

    async fn apply_snapshot_chunk(
        &mut self,
        _request: RequestApplySnapshotChunk,
    ) -> ResponseApplySnapshotChunk {
        Default::default()
    }
}

impl Application for () {}

// impl<T: Transaction, A: ABCFApplication<T>> Application for A {
//
// }
//
