use super::{context::StorageContext, Context, EventContext};
use crate::{
    abci::EventContextImpl,
    module::{Application, Module, ModuleMetadata, RPCs},
    Error, Result,
};
use alloc::collections::BTreeMap;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use tm_protos::abci;

/// ABCF node.
pub struct Node {
    apps: Vec<Box<dyn Application>>,
    metadatas: Vec<ModuleMetadata>,
    rpcs: BTreeMap<String, Box<dyn RPCs>>,
    events: EventContextImpl,
    // event_descriptor: Vec<EventDescriptor>,
    // stateful_storage: Vec<SparseMerkleTree<H, Value, S>>,
    // stateless_storage: Vec<S>,
    // storage_descriptor: Vec<Box<StorageDescriptor>>,
}

impl Node {
    /// create new node for network.
    pub fn new() -> Self {
        Node {
            apps: Vec::new(),
            metadatas: Vec::new(),
            rpcs: BTreeMap::new(),
            events: EventContextImpl::default(),
        }
    }

    /// regist module.
    pub fn regist<M, A, R>(&mut self, m: &M) -> &mut Self
    where
        R: RPCs + 'static,
        A: Application + 'static,
        M: Module<Application = A, RPCs = R>,
    {
        self.apps.push(Box::new(m.application()));
        self.metadatas.push(m.metadata());
        self.rpcs
            .insert(m.metadata().name.to_string(), Box::new(m.rpcs()));
        self
    }
}

impl Node {
    async fn match_and_call_query(&mut self, req: abci::RequestQuery) -> Result<Vec<u8>> {
        // ignore block height for this version.
        // For future, framewrok can roll back storage to special block height,
        // then call rpc.
        let metadata = &self.metadatas[0];

        let splited_path: Vec<&str> = req.path.split('/').collect();
        if splited_path.len() < 1 {
            return Err(Error::QueryPathFormatError);
        }

        if splited_path[0] != "rpc" {
            // Curren version, path only support query rpc. This error is temp error.
            return Err(Error::TempOnlySupportRPC);
        }
        let rpc = self.rpcs.get_mut(splited_path[1]).unwrap();

        let mut context = Context {
            event: None,
            storage: StorageContext {},
        };

        let params = serde_json::from_slice(&req.data).map_err(|_e| Error::JsonParseError)?;

        let resp = rpc
            .call(&mut context, splited_path[1], params)
            .await
            .unwrap();
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
impl tm_abci::Application for Node {
    async fn info(&mut self, _request: abci::RequestInfo) -> abci::ResponseInfo {
        log::info!("info");
        Default::default()
    }

    async fn query(&mut self, req: abci::RequestQuery) -> abci::ResponseQuery {
        log::info!("query");
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
        log::info!("query end");
        resp
    }

    async fn check_tx(&mut self, req: abci::RequestCheckTx) -> abci::ResponseCheckTx {
        log::info!("check_tx");
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.check_tx_events)),
            storage: StorageContext {},
        };

        let mut resp_check_tx: abci::ResponseCheckTx = abci::ResponseCheckTx::default();
        let mut data_map = BTreeMap::new();

        for (index, app) in self.apps.iter_mut().enumerate() {
            let metadata = &self.metadatas[index];
            let resp = app.check_tx(&mut context, &req).await;
            data_map.insert(metadata.name.to_string(), resp.data);
            resp_check_tx.gas_used += resp.gas_used;
            resp_check_tx.gas_wanted += resp.gas_wanted;
            resp_check_tx.codespace = metadata.name.to_string();

            if resp.code != 0 {
                resp_check_tx.code = 1;
                break;
            } else {
                resp_check_tx.code = 0;
            }
        }

        let mut check_tx_events = Vec::with_capacity(events.check_tx_events.len());

        while let Some(e) = events.check_tx_events.pop() {
            check_tx_events.push(e);
        }

        resp_check_tx.events = check_tx_events;
        resp_check_tx.info = String::new();
        resp_check_tx.log = String::new();
        resp_check_tx
    }

    async fn begin_block(&mut self, req: abci::RequestBeginBlock) -> abci::ResponseBeginBlock {
        log::info!("begin_block");
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.begin_block_events)),
            storage: StorageContext {},
        };

        for app in self.apps.iter_mut() {
            app.begin_block(&mut context, &req).await;
        }

        let mut begin_block_events = Vec::with_capacity(events.begin_block_events.len());

        while let Some(e) = events.begin_block_events.pop() {
            begin_block_events.push(e);
        }

        abci::ResponseBeginBlock {
            events: begin_block_events,
        }
    }

    async fn deliver_tx(&mut self, _request: abci::RequestDeliverTx) -> abci::ResponseDeliverTx {
        log::info!("deliver_tx");
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.deliver_tx_events)),
            storage: StorageContext {},
        };

        let mut resp_deliver_tx: abci::ResponseDeliverTx = abci::ResponseDeliverTx::default();
        let mut data_map = BTreeMap::new();

        for (index, app) in self.apps.iter_mut().enumerate() {
            let metadata = &self.metadatas[index];
            let resp = app.deliver_tx(&mut context, &_request).await;
            data_map.insert(metadata.name.to_string(), resp.data);
            resp_deliver_tx.gas_used += resp.gas_used;
            resp_deliver_tx.gas_wanted += resp.gas_wanted;
            resp_deliver_tx.codespace = metadata.name.to_string();

            if resp.code != 0 {
                resp_deliver_tx.code = 1;
                break;
            } else {
                resp_deliver_tx.code = 0;
            }
        }

        let mut deliver_tx_events = Vec::with_capacity(events.deliver_tx_events.len());

        while let Some(e) = events.deliver_tx_events.pop() {
            deliver_tx_events.push(e);
        }

        let data_map_json = serde_json::to_string(&data_map).unwrap();

        resp_deliver_tx.events = deliver_tx_events;
        resp_deliver_tx.info = String::new();
        resp_deliver_tx.log = String::new();
        resp_deliver_tx.data = data_map_json.as_bytes().to_vec();
        resp_deliver_tx
    }

    async fn end_block(&mut self, _request: abci::RequestEndBlock) -> abci::ResponseEndBlock {
        log::info!("deliver_tx");
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.deliver_tx_events)),
            storage: StorageContext {},
        };

        let mut validator_updates_vec = Vec::new();
        let mut resp_end_block: abci::ResponseEndBlock = abci::ResponseEndBlock::default();

        for app in self.apps.iter_mut() {
            let resp = app.end_block(&mut context, &_request).await;
            resp.validator_updates.into_iter().for_each(|v| {
                if !validator_updates_vec.contains(&v) {
                    validator_updates_vec.push(v);
                }
            });
            resp_end_block.consensus_param_updates = resp.consensus_param_updates;
        }

        let mut end_block_events = Vec::with_capacity(events.end_block_events.len());

        while let Some(e) = events.end_block_events.pop() {
            end_block_events.push(e);
        }

        resp_end_block.validator_updates = validator_updates_vec;
        resp_end_block.events = end_block_events;
        resp_end_block
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        module::{types, Application, RPCResponse},
        Genesis, Result,
    };
    use tm_abci::Application as app;
    use tm_protos::abci::RequestDeliverTx;
    use tokio::runtime::Runtime;

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
        ) -> Result<RPCResponse<'_, serde_json::Value>> {
            Ok(RPCResponse::default())
        }
    }

    pub struct MockModule {}

    impl Module for MockModule {
        type Application = MockApplicaion;
        type RPCs = MockRPCs;

        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata {
                name: "mock".to_string(),
                version: "0.1.0".to_string(),
                impl_version: "0.1.0".to_string(),
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

    pub struct MockApplicaion2 {}

    #[async_trait::async_trait]
    impl Application for MockApplicaion2 {
        async fn deliver_tx(
            &mut self,
            _context: &mut Context,
            _req: &RequestDeliverTx,
        ) -> types::ResponseDeliverTx {
            let mut resp: types::ResponseDeliverTx = types::ResponseDeliverTx::default();
            resp.code = 1;
            resp.data = "error from me".as_bytes().to_vec();
            resp.gas_wanted = 1;
            resp.gas_used = 20;
            resp
        }
    }

    pub struct MockRPCs2 {}

    #[async_trait::async_trait]
    impl RPCs for MockRPCs2 {
        async fn call(
            &mut self,
            _context: &mut Context,
            _method: &str,
            _params: serde_json::Value,
        ) -> Result<RPCResponse<'_, serde_json::Value>> {
            Ok(RPCResponse::default())
        }
    }

    pub struct MockModule2 {}

    impl Module for MockModule2 {
        type Application = MockApplicaion2;
        type RPCs = MockRPCs2;

        fn metadata(&self) -> ModuleMetadata {
            ModuleMetadata {
                name: "mock2".to_string(),
                version: "0.1.0".to_string(),
                impl_version: "0.1.0".to_string(),
                genesis: Genesis { target_hight: 1 },
            }
        }

        fn application(&self) -> Self::Application {
            MockApplicaion2 {}
        }

        fn rpcs(&self) -> Self::RPCs {
            MockRPCs2 {}
        }
    }

    #[test]
    fn test_mock() {
        let mut node = Node::new();
        let module = MockModule {};
        let module2 = MockModule2 {};
        node.regist(&module);
        node.regist(&module2);

        let request: abci::RequestDeliverTx = abci::RequestDeliverTx::default();
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let resp = node.deliver_tx(request).await;
            assert_eq!(resp.code, 1);
            assert_eq!(resp.codespace, "mock2".to_string());
            assert_eq!(resp.gas_used, 20);
            assert_eq!(resp.gas_wanted, 1);
            assert_eq!(resp.events.len(), 0);
            {
                let mut data_map = BTreeMap::new();
                data_map.insert("mock", "".as_bytes().to_vec());
                data_map.insert("mock2", "error from me".as_bytes().to_vec());
                let data_map_json = serde_json::to_string(&data_map)
                    .unwrap()
                    .as_bytes()
                    .to_vec();
                assert_eq!(resp.data, data_map_json);
            }
        });
    }
}
