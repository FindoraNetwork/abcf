use core::marker::PhantomData;

use alloc::{boxed::Box, string::String, vec::Vec};

use bs3::Store;
use digest::Digest;
use tm_protos::abci::{
    RequestBeginBlock, RequestInfo, RequestInitChain, RequestQuery, ResponseBeginBlock,
    ResponseInfo, ResponseInitChain, ResponseQuery,
};

use crate::{Error, Merkle, Module, ModuleError, ModuleResult, Storage};

use super::{
    prelude::{Application, RPCs},
    Context, RPCContext,
};

pub struct Node<S, D, Sl, Sf, M>
where
    S: Store,
    D: Digest,
    Sl: Storage<S>,
    Sf: Storage<S> + Merkle<D>,
    M: Module + Application<Sl, Sf> + RPCs<Sl, Sf>,
{
    stateless: Sl,
    stateful: Sf,
    marker_s: PhantomData<S>,
    marker_d: PhantomData<D>,
    module: M,
}

impl<S, D, Sl, Sf, M> Node<S, D, Sl, Sf, M>
where
    S: Store,
    D: Digest,
    Sl: Storage<S>,
    Sf: Storage<S> + Merkle<D>,
    M: Module + Application<Sl, Sf> + RPCs<Sl, Sf>,
{
    pub fn new(stateless: Sl, stateful: Sf, module: M) -> Self {
        Self {
            stateful,
            stateless,
            module,
            marker_s: PhantomData,
            marker_d: PhantomData,
        }
    }

    async fn call_rpc(&mut self, sub_path: Option<&str>, req: &[u8]) -> ModuleResult<(Vec<u8>, Vec<u8>)> {
        let mut ctx = RPCContext {
            events: None,
            stateless: &mut self.stateless,
            stateful: &self.stateful,
        };
        let method = sub_path.ok_or(ModuleError {
            namespace: String::from("abcf.rpc"),
            error: Error::QueryPathFormatError,
        })?;
        let params = serde_json::from_slice(req).map_err(|e| ModuleError {
            namespace: String::from("abcf.rpc"),
            error: Error::JsonError(e),
        })?;

        let result = self.module.call(&mut ctx, method, params).await?;

        let value = serde_json::to_vec(&result).map_err(|e| ModuleError {
            namespace: String::from("abcf.rpc"),
            error: Error::JsonError(e),
        })?;

        let key = method.as_bytes().to_vec();

        Ok((key, value))
    }
}

#[async_trait::async_trait]
impl<S, D, Sl, Sf, M> tm_abci::Application for Node<S, D, Sl, Sf, M>
where
    S: Store,
    D: Digest + Send,
    Sl: Storage<S>,
    Sf: Storage<S> + Merkle<D>,
    M: Module + Application<Sl, Sf> + RPCs<Sl, Sf>,
{
    async fn init_chain(&mut self, _request: RequestInitChain) -> ResponseInitChain {
        let mut resp = ResponseInitChain::default();

        resp.app_hash = self.stateful.root().expect("get app hash failed").to_vec();

        resp
    }

    async fn info(&mut self, _request: RequestInfo) -> ResponseInfo {
        let mut resp = ResponseInfo::default();

        resp.version = String::from(self.module.metadata().impl_version);
        resp.app_version = self.module.metadata().version;

        // compare height.
        let stateful_height = self
            .stateful
            .height()
            .expect("get current height for stateful storage failed");
        let stateless_height = self
            .stateless
            .height()
            .expect("get current height for stateless storage failed");

        if stateless_height >= stateful_height {
            self.stateless
                .rollback(stateful_height)
                .expect("rollback to height failed.");
            resp.last_block_height = stateful_height;
            resp.last_block_app_hash = self.stateful.root().expect("get app hash failed").to_vec();
        } else {
            self.stateful
                .rollback(stateless_height)
                .expect("rollback to height failed.");
            resp.last_block_height = stateful_height;
            resp.last_block_app_hash = self.stateful.root().expect("get app hash failed").to_vec();
        }

        resp
    }

    async fn query(&mut self, request: RequestQuery) -> ResponseQuery {
        let mut paths = request.path.splitn(2, "/");

        let mut resp = ResponseQuery::default();

        let res = match paths.next() {
            Some("rpc") => self.call_rpc(paths.next(), request.data.as_ref()).await,
            Some("stateless") => Err(ModuleError {
                namespace: String::from("abcf"),
                error: Error::QueryPathFormatError,
            }),
            Some("stateful") => Err(ModuleError {
                namespace: String::from("abcf"),
                error: Error::QueryPathFormatError,
            }),
            Some(_) | None => Err(ModuleError {
                namespace: String::from("abcf"),
                error: Error::QueryPathFormatError,
            }),
        };

        match res {
            Ok((k, v)) => {
                resp.key = k;
                resp.value = v;
            }
            Err(e) => {
                resp.code = e.error.code();
                resp.log = e.error.message();
                resp.codespace = e.namespace;
            }
        }

        resp
    }

    async fn begin_block(&mut self, _request: RequestBeginBlock) -> ResponseBeginBlock {
        Default::default()
    }
}
