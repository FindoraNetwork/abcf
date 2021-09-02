use core::any::Any;

use super::{context::StorageContext, Context, EventContext};
use crate::{
    framework::{context::CallContext, EventContextImpl},
    module::{Application, Module, ModuleMetadata, RPCs},
    Error, ModuleError, ModuleResult, Result,
};
use alloc::collections::BTreeMap;
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use core::mem;
use tm_protos::abci;

/// ABCF node.
pub struct Node {
    apps: Vec<Box<dyn Application>>,
    metadatas: Vec<ModuleMetadata>,
    rpcs: Vec<Box<dyn RPCs>>,
    name_index: BTreeMap<String, usize>,
    events: EventContextImpl,
    callables: Vec<Box<dyn Any + Send + Sync>>,
}

impl Node {
    /// create new node for network.
    pub fn new() -> Self {
        Node {
            apps: Vec::new(),
            metadatas: Vec::new(),
            rpcs: Vec::new(),
            name_index: BTreeMap::new(),
            events: EventContextImpl::default(),
            callables: Vec::new(),
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
        self.rpcs.push(Box::new(m.rpcs()));
        self.callables.push(Box::new(m.callable()));

        self.name_index
            .insert(m.metadata().name, self.apps.len() - 1);
        self
    }
}

impl Node {
    async fn match_and_call_query(&mut self, req: abci::RequestQuery) -> ModuleResult<Vec<u8>> {
        // ignore block height for this version.
        // For future, framewrok can roll back storage to special block height,
        // then call rpc.

        const ABCF_RPC_NAMESPAC: &str = "abcf.rpc";

        async fn call_rpc(
            context: &mut Context<'_>,
            method: &str,
            rpc: &mut dyn RPCs,
            req: Vec<u8>,
        ) -> Result<Vec<u8>> {
            let params = serde_json::from_slice(&req)?;

            let resp = rpc.call(context, method, params).await?;

            Ok(match resp {
                Some(v) => serde_json::to_vec(&v)?,
                None => Vec::new(),
            })
        }

        let splited_path: Vec<&str> = req.path.split('/').collect();
        if splited_path.len() < 2 {
            return Err(ModuleError::new(
                ABCF_RPC_NAMESPAC,
                Error::QueryPathFormatError,
            ));
        }

        if splited_path[0] != "rpc" {
            // Curren version, path only support query rpc. This error is temp error.
            return Err(ModuleError::new(
                ABCF_RPC_NAMESPAC,
                Error::TempOnlySupportRPC,
            ));
        }

        let module_name = splited_path[1];
        let method_name = splited_path[2];

        if let Some(rpc_index) = self.name_index.get(module_name) {
            if let Some(rpc) = self.rpcs.get_mut(*rpc_index) {
                let mut context = Context {
                    event: None,
                    storage: StorageContext {},
                    calls: CallContext {
                        name_index: &self.name_index,
                        calls: &mut self.callables,
                    },
                };

                log::info!("rpc query {}: {}", module_name, method_name);
                match call_rpc(&mut context, method_name, rpc.as_mut(), req.data).await {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ModuleError::new(module_name, e)),
                }
            } else {
                Err(ModuleError::new(ABCF_RPC_NAMESPAC, Error::NoModule))
            }
        } else {
            Err(ModuleError::new(ABCF_RPC_NAMESPAC, Error::NoModule))
        }
    }
}

#[async_trait::async_trait]
impl tm_abci::Application for Node {
    async fn info(&mut self, _request: abci::RequestInfo) -> abci::ResponseInfo {
        Default::default()
    }

    async fn query(&mut self, req: abci::RequestQuery) -> abci::ResponseQuery {
        let mut resp = abci::ResponseQuery::default();

        let result = self.match_and_call_query(req).await;

        match result {
            Ok(bytes) => resp.value = bytes,
            Err(e) => {
                if let Error::RPCApplicationError(code, error) = e.error {
                    resp.code = code;
                    resp.log = error;
                    resp.codespace = e.namespace;
                } else {
                    resp.code = e.error.to_code();
                    resp.log = alloc::format!("{:?}", e.error);
                    resp.codespace = e.namespace;
                }
            }
        }
        resp
    }

    async fn check_tx(&mut self, req: abci::RequestCheckTx) -> abci::ResponseCheckTx {
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.check_tx_events)),
            storage: StorageContext {},
            calls: CallContext {
                name_index: &self.name_index,
                calls: &mut self.callables,
            },
        };

        let mut resp = abci::ResponseCheckTx::default();
        let mut data_map = BTreeMap::new();

        for (index, app) in self.apps.iter_mut().enumerate() {
            let metadata = &self.metadatas[index];
            match app.check_tx(&mut context, &req).await {
                Ok(module_resp) => {
                    data_map.insert(metadata.name.clone(), module_resp.data);
                    resp.gas_used += module_resp.gas_used;
                    resp.gas_wanted += module_resp.gas_wanted;
                }
                Err(e) => {
                    resp.codespace = metadata.name.clone();
                    match e {
                        Error::ABCIApplicationError(code, message) => {
                            resp.code = code;
                            resp.log = message;
                        }
                        _ => {
                            resp.code = e.to_code();
                            resp.log = alloc::format!("{:?}", e);
                        }
                    }
                    return resp;
                }
            }
        }

        let check_tx_events = mem::replace(&mut events.check_tx_events, Vec::new());

        match serde_json::to_vec(&data_map) {
            Ok(v) => resp.data = v,
            Err(e) => {
                let err = Error::JsonError(e);
                resp.code = err.to_code();
                resp.log = alloc::format!("{:?}", err);
                resp.codespace = String::from("abcf.application");
            }
        }

        resp.events = check_tx_events;
        resp
    }

    async fn begin_block(&mut self, req: abci::RequestBeginBlock) -> abci::ResponseBeginBlock {
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.begin_block_events)),
            storage: StorageContext {},
            calls: CallContext {
                name_index: &self.name_index,
                calls: &mut self.callables,
            },
        };

        for app in self.apps.iter_mut() {
            app.begin_block(&mut context, &req).await;
        }

        let begin_block_events = mem::replace(&mut events.begin_block_events, Vec::new());

        abci::ResponseBeginBlock {
            events: begin_block_events,
        }
    }

    async fn deliver_tx(&mut self, req: abci::RequestDeliverTx) -> abci::ResponseDeliverTx {
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.deliver_tx_events)),
            storage: StorageContext {},
            calls: CallContext {
                name_index: &self.name_index,
                calls: &mut self.callables,
            },
        };

        let mut resp: abci::ResponseDeliverTx = abci::ResponseDeliverTx::default();
        let mut data_map = BTreeMap::new();

        for (index, app) in self.apps.iter_mut().enumerate() {
            let metadata = &self.metadatas[index];
            match app.deliver_tx(&mut context, &req).await {
                Ok(module_resp) => {
                    data_map.insert(metadata.name.to_string(), module_resp.data);
                    resp.gas_used += module_resp.gas_used;
                    resp.gas_wanted += module_resp.gas_wanted;
                }
                Err(e) => {
                    resp.codespace = metadata.name.clone();
                    match e {
                        Error::ABCIApplicationError(code, message) => {
                            resp.code = code;
                            resp.log = message;
                        }
                        _ => {
                            resp.code = e.to_code();
                            resp.log = alloc::format!("{:?}", e);
                        }
                    }
                    return resp;
                }
            }
        }

        let deliver_tx_events = mem::replace(&mut events.deliver_tx_events, Vec::new());

        match serde_json::to_vec(&data_map) {
            Ok(v) => resp.data = v,
            Err(e) => {
                let err = Error::JsonError(e);
                resp.code = err.to_code();
                resp.log = alloc::format!("{:?}", err);
                resp.codespace = String::from("abcf.application");
            }
        }

        resp.events = deliver_tx_events;
        resp
    }

    async fn end_block(&mut self, _request: abci::RequestEndBlock) -> abci::ResponseEndBlock {
        let events = &mut self.events;

        // construct context for call.
        let mut context = Context {
            event: Some(EventContext::new(&mut events.deliver_tx_events)),
            storage: StorageContext {},
            calls: CallContext {
                name_index: &self.name_index,
                calls: &mut self.callables,
            },
        };

        let mut validator_updates_vec = Vec::new();
        let mut resp: abci::ResponseEndBlock = abci::ResponseEndBlock::default();

        for app in self.apps.iter_mut() {
            let module_resp = app.end_block(&mut context, &_request).await;
            module_resp.validator_updates.into_iter().for_each(|v| {
                if !validator_updates_vec.contains(&v) {
                    validator_updates_vec.push(v);
                }
            });
            resp.consensus_param_updates = resp.consensus_param_updates;
        }

        let end_block_events = mem::replace(&mut events.end_block_events, Vec::new());

        resp.validator_updates = validator_updates_vec;
        resp.events = end_block_events;
        resp
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        module::{types, Application},
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
        ) -> Result<Option<serde_json::Value>> {
            Ok(Default::default())
        }
    }

    pub struct MockModule {}

    impl Module for MockModule {
        type Application = MockApplicaion;
        type RPCs = MockRPCs;
        type Callable = ();

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

        fn callable(&self) -> Self::Callable {
            ()
        }
    }

    pub struct MockApplicaion2 {}

    #[async_trait::async_trait]
    impl Application for MockApplicaion2 {
        async fn deliver_tx(
            &mut self,
            _context: &mut Context,
            _req: &RequestDeliverTx,
        ) -> Result<types::ResponseDeliverTx> {
            let mut resp: types::ResponseDeliverTx = types::ResponseDeliverTx::default();
            resp.data = "error from me".as_bytes().to_vec();
            resp.gas_wanted = 1;
            resp.gas_used = 20;
            Err(Error::ABCIApplicationError(1, String::from("mock error")))
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
        ) -> Result<Option<serde_json::Value>> {
            Ok(Some(Default::default()))
        }
    }

    pub struct MockModule2 {}

    impl Module for MockModule2 {
        type Application = MockApplicaion2;
        type RPCs = MockRPCs2;
        type Callable = ();

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

        fn callable(&self) -> Self::Callable {
            ()
        }
    }

    #[test]
    fn test_mock_failed_fallback() {
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
            assert_eq!(resp.gas_used, 0);
            assert_eq!(resp.gas_wanted, 0);
            assert_eq!(resp.events.len(), 0);
            // {
            // let mut data_map = BTreeMap::new();
            // data_map.insert("mock", "".as_bytes().to_vec());
            // data_map.insert("mock2", "error from me".as_bytes().to_vec());
            // let data_map_json = serde_json::to_string(&data_map)
            //     .unwrap()
            //     .as_bytes()
            //     .to_vec();
            // assert_eq!(resp.data, data_map_json);
            // }
        });
    }
}
