use crate::{
    module::{Application, Module, ModuleMetadata, RPCs},
    Error, Result,
};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    vec::Vec,
};
use tm_protos::abci;

pub struct Node {
    apps: Vec<Box<dyn Application>>,
    metadatas: Vec<ModuleMetadata>,
    rpcs: Vec<Box<dyn RPCs>>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            apps: Vec::new(),
            metadatas: Vec::new(),
            rpcs: Vec::new(),
        }
    }

    pub fn regist<M, A, R>(&mut self, m: &M)
    where
        R: RPCs + 'static,
        A: Application + 'static,
        M: Module<Application = A, RPCs = R>,
    {
        self.metadatas.push(m.metadata());
        self.apps.push(Box::new(m.application()));
        self.rpcs.push(Box::new(m.rpcs()));
    }

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

        let params = serde_json::from_slice(&req.data).map_err(|_e| Error::JsonParseError)?;
        let resp = rpc.call("", params).await;
        if resp.code != 0 {
            return Err(Error::RPRApplicationError(resp.code, metadata.name.clone()));
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

    use crate::module::rpcs::Response;

    use super::*;

    pub struct MockApplicaion {}

    impl Application for MockApplicaion {}

    pub struct MockRPCs {}

    #[async_trait::async_trait]
    impl RPCs for MockRPCs {
        async fn call(
            &mut self,
            _method: &str,
            _params: serde_json::Value,
        ) -> Response<'_, serde_json::Value> {
            Response::default()
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
            }
        }

        fn application(&self) -> Self::Application {
            MockApplicaion {}
        }

        fn rpcs(&self) -> Self::RPCs {
            MockRPCs {}
        }
    }
}
