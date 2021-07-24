use crate::module::{Application, Module, ModuleMetadata};
use alloc::{boxed::Box, string::String, vec::Vec};
use tm_protos::abci;

pub struct Node {
    apps: Box<dyn Application>,
    metadatas: ModuleMetadata,
}

impl Node {
    pub fn regist<M, A>(&mut self, m: &M)
    where
        A: Application + 'static,
        M: Module<Application = A>,
    {
        self.metadatas = m.metadata();
        self.apps = Box::new(m.application());
    }
}

#[async_trait::async_trait]
impl tm_abci::Application for Node {
    async fn check_tx(&mut self, req: abci::RequestCheckTx) -> abci::ResponseCheckTx {
        let resp = self.apps.check_tx(&req).await;
        abci::ResponseCheckTx {
            code: resp.code,
            data: resp.data,
            log: String::new(),
            info: String::new(),
            gas_wanted: resp.gas_wanted,
            gas_used: resp.gas_used,
            events: Vec::new(),
            codespace: self.metadatas.name.clone(),
        }
    }

    async fn begin_block(&mut self, req: abci::RequestBeginBlock) -> abci::ResponseBeginBlock {
        self.apps.begin_block(&req).await;
        abci::ResponseBeginBlock { events: Vec::new() }
    }

    async fn deliver_tx(&mut self, _request: abci::RequestDeliverTx) -> abci::ResponseDeliverTx {
        let resp = self.apps.deliver_tx(&_request).await;
        abci::ResponseDeliverTx {
            code: resp.code,
            data: resp.data,
            log: String::new(),
            info: String::new(),
            gas_wanted: resp.gas_wanted,
            gas_used: resp.gas_used,
            events: Vec::new(),
            codespace: self.metadatas.name.clone(),
        }
    }

    async fn end_block(&mut self, _request: abci::RequestEndBlock) -> abci::ResponseEndBlock {
        let resp = self.apps.end_block(&_request).await;
        abci::ResponseEndBlock {
            validator_updates: resp.validator_updates,
            consensus_param_updates: resp.consensus_param_updates,
            events: Vec::new(),
        }
    }
}
