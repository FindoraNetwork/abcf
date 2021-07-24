use crate::module::{Application, Module, ModuleMetadata, RPCs};
use alloc::{boxed::Box, string::String, vec::Vec};
use tm_protos::abci;

pub struct Node {
    apps: Vec<Box<dyn Application>>,
    metadatas: Vec<ModuleMetadata>,
    // rpcs: Box<dyn RPCs>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            apps: Vec::new(),
            metadatas: Vec::new(),
        }
    }

    pub fn regist<M, A>(&mut self, m: &M)
    where
        // R: RPCs + 'static,
        A: Application + 'static,
        M: Module<Application = A>,
    {
        self.metadatas.push(m.metadata());
        self.apps.push(Box::new(m.application()));
        // self.rpcs = Box::new(m.rpcs());
    }
}

#[async_trait::async_trait]
impl tm_abci::Application for Node {
    async fn check_tx(&mut self, req: abci::RequestCheckTx) -> abci::ResponseCheckTx {
        let app = &mut self.apps[0];
        let metadata = &self.metadatas[0];
        let resp = app.check_tx(&req).await;
        abci::ResponseCheckTx {
            code: resp.code,
            data: resp.data,
            log: String::new(),
            info: String::new(),
            gas_wanted: resp.gas_wanted,
            gas_used: resp.gas_used,
            events: Vec::new(),
            codespace: metadata.name.clone(),
        }
    }

    async fn begin_block(&mut self, req: abci::RequestBeginBlock) -> abci::ResponseBeginBlock {
        let app = &mut self.apps[0];
        app.begin_block(&req).await;
        abci::ResponseBeginBlock { events: Vec::new() }
    }

    async fn deliver_tx(&mut self, _request: abci::RequestDeliverTx) -> abci::ResponseDeliverTx {
        let app = &mut self.apps[0];
        let metadata = &self.metadatas[0];
        let resp = app.deliver_tx(&_request).await;
        abci::ResponseDeliverTx {
            code: resp.code,
            data: resp.data,
            log: String::new(),
            info: String::new(),
            gas_wanted: resp.gas_wanted,
            gas_used: resp.gas_used,
            events: Vec::new(),
            codespace: metadata.name.clone(),
        }
    }

    async fn end_block(&mut self, _request: abci::RequestEndBlock) -> abci::ResponseEndBlock {
        let app = &mut self.apps[0];
        let resp = app.end_block(&_request).await;
        abci::ResponseEndBlock {
            validator_updates: resp.validator_updates,
            consensus_param_updates: resp.consensus_param_updates,
            events: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;

    use super::*;

    pub struct MockApplicaion {}
    
    impl Application for MockApplicaion {}

    pub struct MockModule {}

    impl Module for MockModule {
        type Application = MockApplicaion;

        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata {
                name: "mock".to_string(),
                version: "0.1.0".to_string(),
                impl_version: "0.1.0".to_string()
            }
        }

        fn application(&self) -> Self::Application {
            MockApplicaion {}
        }
    }
}
