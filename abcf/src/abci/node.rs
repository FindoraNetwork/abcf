use super::{context::StorageContext, Context, EventContext};
use crate::{
    abci::EventContextImpl,
    module::{Application, Module, ModuleMetadata, RPCs},
    Error, Result,
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use tm_protos::abci;

/// ABCF node.
pub struct Node<'a> {
    apps: Vec<Box<dyn Application>>,
    metadatas: Vec<ModuleMetadata<'a>>,
    rpcs: Vec<Box<dyn RPCs>>,
    events: EventContextImpl,
    // event_descriptor: Vec<EventDescriptor>,
    // stateful_storage: Vec<SparseMerkleTree<H, Value, S>>,
    // stateless_storage: Vec<S>,
    // storage_descriptor: Vec<Box<StorageDescriptor>>,
}

impl<'a> Node<'a> {
    /// create new node for network.
    pub fn new() -> Self {
        Node {
            apps: Vec::new(),
            metadatas: Vec::new(),
            rpcs: Vec::new(),
            events: EventContextImpl::default(),
        }
    }

    /// regist module.
    pub fn regist<M, A, R>(&mut self, m: &'a M)
    where
        R: RPCs + 'static,
        A: Application + 'static,
        M: Module<Application = A, RPCs = R>,
    {
        self.apps.push(Box::new(m.application()));
        self.metadatas.push(m.metadata());
        self.rpcs.push(Box::new(m.rpcs()));
    }
}

impl<'a> Node<'a> {
    async fn match_and_call_query(&mut self, req: abci::RequestQuery) -> Result<Vec<u8>> {
        // ignore block height for this version.
        // For future, framewrok can roll back storage to special block height,
        // then call rpc.
        let rpc = &mut self.rpcs[0];
        let metadata = &self.metadatas[0];

        let splited_path: Vec<&str> = req.path.split('/').collect();
        if splited_path.len() < 1 {
            return Err(Error::QueryPathFormatError);
        }

        if splited_path[0] != "rpc" {
            // Curren version, path only support query rpc. This error is temp error.
            return Err(Error::TempOnlySupportRPC);
        }

        let mut context = Context {
            event: None,
            storage: StorageContext {},
        };

        let params = serde_json::from_slice(&req.data).map_err(|_e| Error::JsonParseError)?;
        let resp = rpc.call(&mut context, splited_path[1], params).await;
        if resp.code != 0 {
            return Err(Error::RPRApplicationError(
                resp.code,
                metadata.name.to_string(),
            ));
        }

        if resp.data.is_none() {
            return Ok(Vec::new());
        }

        let resp_bytes = serde_json::to_vec(&resp.data).map_err(|_e| Error::JsonDumpError)?;
        Ok(resp_bytes)
    }
}

#[async_trait::async_trait]
impl<'a> tm_abci::Application for Node<'a> {
    async fn query(&mut self, req: abci::RequestQuery) -> abci::ResponseQuery {
        let mut resp = abci::ResponseQuery::default();

        match self.match_and_call_query(req).await {
            Ok(bytes) => resp.value = bytes,
            Err(e) => {
                if let Error::RPRApplicationError(code, codespace) = e {
                    resp.code = code;
                    resp.codespace = codespace;
                } else {
                    resp.code = e.to_code();
                    resp.codespace = "abcf.rpc".to_string();
                }
            }
        }
        resp
    }

    async fn check_tx(&mut self, req: abci::RequestCheckTx) -> abci::ResponseCheckTx {
        let app = &mut self.apps[0];
        let metadata = &self.metadatas[0];
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.check_tx_events)),
            storage: StorageContext {},
        };

        let resp = app.check_tx(&mut context, &req).await;

        let mut check_tx_events = Vec::with_capacity(events.check_tx_events.len());

        while let Some(e) = events.check_tx_events.pop() {
            check_tx_events.push(e);
        }

        abci::ResponseCheckTx {
            code: resp.code,
            data: resp.data,
            log: String::new(),
            info: String::new(),
            gas_wanted: resp.gas_wanted,
            gas_used: resp.gas_used,
            events: check_tx_events,
            codespace: metadata.name.to_string(),
        }
    }

    async fn begin_block(&mut self, req: abci::RequestBeginBlock) -> abci::ResponseBeginBlock {
        let app = &mut self.apps[0];
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.begin_block_events)),
            storage: StorageContext {},
        };

        app.begin_block(&mut context, &req).await;

        let mut begin_block_events = Vec::with_capacity(events.begin_block_events.len());

        while let Some(e) = events.begin_block_events.pop() {
            begin_block_events.push(e);
        }

        abci::ResponseBeginBlock {
            events: begin_block_events,
        }
    }

    async fn deliver_tx(&mut self, _request: abci::RequestDeliverTx) -> abci::ResponseDeliverTx {
        let app = &mut self.apps[0];
        let metadata = &self.metadatas[0];
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.deliver_tx_events)),
            storage: StorageContext {},
        };

        let resp = app.deliver_tx(&mut context, &_request).await;

        let mut deliver_tx_events = Vec::with_capacity(events.deliver_tx_events.len());

        while let Some(e) = events.deliver_tx_events.pop() {
            deliver_tx_events.push(e);
        }

        abci::ResponseDeliverTx {
            code: resp.code,
            data: resp.data,
            log: String::new(),
            info: String::new(),
            gas_wanted: resp.gas_wanted,
            gas_used: resp.gas_used,
            events: deliver_tx_events,
            codespace: metadata.name.to_string(),
        }
    }

    async fn end_block(&mut self, _request: abci::RequestEndBlock) -> abci::ResponseEndBlock {
        let app = &mut self.apps[0];
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.deliver_tx_events)),
            storage: StorageContext {},
        };

        let resp = app.end_block(&mut context, &_request).await;

        let mut end_block_events = Vec::with_capacity(events.end_block_events.len());

        while let Some(e) = events.end_block_events.pop() {
            end_block_events.push(e);
        }
        abci::ResponseEndBlock {
            validator_updates: resp.validator_updates,
            consensus_param_updates: resp.consensus_param_updates,
            events: end_block_events,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{module::RPCResponse, Event, Genesis, Result};

    pub struct MockApplicaion {}

    impl Application for MockApplicaion {}

    pub struct MockRPCs {}

    #[async_trait::async_trait]
    impl RPCs for MockRPCs {
        async fn call(
            &mut self,
            _context: &mut Context,
            _method: &str,
            _params: serde_json::Value,
        ) -> RPCResponse<'_, serde_json::Value> {
            RPCResponse::default()
        }
    }

    #[derive(Debug)]
    pub enum MockEvent {
        Unknown,
    }

    impl Event for MockEvent {
        fn to_abci_event(&self) -> Result<abci::Event> {
            abci::Event::default()
        }

        fn name(&self) -> &str {
            "1000"
        }

        fn all() -> &'static [&'static str] {
            &[]
        }
    }

    pub struct MockModule {}

    impl Module for MockModule {
        type Application = MockApplicaion;
        type RPCs = MockRPCs;
        type Event = MockEvent;

        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata {
                name: "mock",
                version: "0.1.0",
                impl_version: "0.1.0",
                genesis: Genesis { target_hight: 1 },
            }
        }

        fn application(&self) -> Self::Application {
            MockApplicaion {}
        }

        fn rpcs(&self) -> Self::RPCs {
            MockRPCs {}
        }
    }

    #[test]
    fn test_mock() {
        let mut node = Node::new();
        let module = MockModule {};
        node.regist(&module);
    }
}
