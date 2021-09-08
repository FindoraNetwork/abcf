use core::{marker::PhantomData, mem};

use alloc::{boxed::Box, string::String, vec::Vec};

use digest::Digest;
use tm_protos::abci::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, RequestInfo,
    RequestInitChain, RequestQuery, ResponseBeginBlock, ResponseCheckTx, ResponseCommit,
    ResponseDeliverTx, ResponseEndBlock, ResponseInfo, ResponseInitChain, ResponseQuery,
};

use crate::{
    module::StorageTransaction, Error, Merkle, Module, ModuleError, ModuleResult, Storage,
};

use super::{
    context::TContext,
    prelude::{Application, RPCs, Tree},
    AContext, EventContext, EventContextImpl, RContext,
};

pub struct Node<D, Sl, Sf, M>
where
    D: Digest,
    Sl: Storage + StorageTransaction + Tree,
    Sf: Storage + StorageTransaction + Tree + Merkle<D>,
    M: Module + Application<Sl, Sf> + RPCs<Sl, Sf>,
{
    stateless: Sl,
    stateful: Sf,
    marker_d: PhantomData<D>,
    module: M,
    events: EventContextImpl,
}

impl<D, Sl, Sf, M> Node<D, Sl, Sf, M>
where
    D: Digest,
    Sl: Storage + StorageTransaction + Tree,
    Sf: Storage + StorageTransaction + Tree + Merkle<D>,
    M: Module + Application<Sl, Sf> + RPCs<Sl, Sf>,
{
    pub fn new(stateless: Sl, stateful: Sf, module: M) -> Self {
        Self {
            stateful,
            stateless,
            module,
            marker_d: PhantomData,
            events: EventContextImpl::default(),
        }
    }

    async fn call_rpc(
        &mut self,
        sub_path: Option<&str>,
        req: &[u8],
    ) -> ModuleResult<(Vec<u8>, Vec<u8>)> {
        let mut ctx = RContext {
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

    async fn call_store(
        &self,
        store: &impl Tree,
        sub_path: Option<&str>,
        height: i64,
    ) -> ModuleResult<(Vec<u8>, Vec<u8>)> {
        let key = sub_path.ok_or(ModuleError {
            namespace: String::from("abcf.store"),
            error: Error::QueryPathFormatError,
        })?;
        let value = store.get(key, height)?;
        Ok((key.as_bytes().to_vec(), value))
    }
}

#[async_trait::async_trait]
impl<D, Sl, Sf, M> tm_abci::Application for Node<D, Sl, Sf, M>
where
    D: Digest + Send + Sync,
    Sl: Storage + StorageTransaction + Tree,
    Sf: Storage + StorageTransaction + Tree + Merkle<D>,
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

        let sub_path = paths.next();

        let res = match paths.next() {
            Some("rpc") => self.call_rpc(sub_path, request.data.as_ref()).await,
            Some("stateful") => {
                self.call_store(&self.stateful, sub_path, request.height)
                    .await
            }
            Some("stateless") => {
                self.call_store(&self.stateless, sub_path, request.height)
                    .await
            }
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

    async fn check_tx(&mut self, req: RequestCheckTx) -> ResponseCheckTx {
        let mut resp = ResponseCheckTx::default();

        let check_tx_events = &mut self.events.check_tx_events;

        let mut stateful_tx = self.stateful.transaction();
        let mut stateless_tx = self.stateless.transaction();

        let mut ctx = TContext {
            events: EventContext::new(check_tx_events),
            stateless: &mut stateless_tx,
            stateful: &mut stateful_tx,
        };

        let result = self.module.check_tx(&mut ctx, req).await;

        match result {
            Ok(v) => {
                resp.data = v.data;
                resp.gas_wanted = v.gas_wanted;
                resp.gas_used = v.gas_used;
            }
            Err(e) => {
                resp.code = e.error.code();
                resp.log = e.error.message();
                resp.codespace = e.namespace;
            }
        }

        resp
    }

    async fn begin_block(&mut self, req: RequestBeginBlock) -> ResponseBeginBlock {
        let mut resp = ResponseBeginBlock::default();

        let begin_block_events = &mut self.events.begin_block_events;

        if let Some(header) = &req.header {
            if header.height
                != self
                    .stateful
                    .height()
                    .expect("Panic! Can't read height when generalize new block")
            {
                self.stateful
                    .rollback(header.height)
                    .expect("Panic! Can't rollback height when generalize new block");
            }
        }

        if let Some(header) = &req.header {
            if header.height
                != self
                    .stateless
                    .height()
                    .expect("Panic! Can't read height when generalize new block")
            {
                self.stateless
                    .rollback(header.height)
                    .expect("Panic! Can't rollback height when generalize new block");
            }
        }

        let mut ctx = AContext {
            events: EventContext::new(begin_block_events),
            stateful: &mut self.stateful,
            stateless: &mut self.stateless,
        };

        self.module.begin_block(&mut ctx, req).await;

        let events = mem::replace(begin_block_events, Vec::new());

        resp.events = events;

        resp
    }

    async fn deliver_tx(&mut self, req: RequestDeliverTx) -> ResponseDeliverTx {
        let mut resp = ResponseDeliverTx::default();

        let deliver_tx_events = &mut self.events.deliver_tx_events;

        let mut stateful_tx = self.stateful.transaction();
        let mut stateless_tx = self.stateless.transaction();

        let mut ctx = TContext {
            events: EventContext::new(deliver_tx_events),
            stateless: &mut stateless_tx,
            stateful: &mut stateful_tx,
        };

        let result = self.module.deliver_tx(&mut ctx, req).await;

        let stateful_cache = Sf::cache(stateful_tx);
        let stateless_cache = Sl::cache(stateless_tx);

        match result {
            Ok(v) => {
                resp.data = v.data;
                resp.gas_wanted = v.gas_wanted;
                resp.gas_used = v.gas_used;

                self.stateful.execute(stateful_cache);
                self.stateless.execute(stateless_cache);
            }
            Err(e) => {
                resp.code = e.error.code();
                resp.log = e.error.message();
                resp.codespace = e.namespace;
            }
        }

        // TODO: add config for module to add or drop events.

        let events = mem::replace(deliver_tx_events, Vec::new());

        resp.events = events;

        resp
    }

    async fn end_block(&mut self, req: RequestEndBlock) -> ResponseEndBlock {
        let mut resp = ResponseEndBlock::default();

        let end_block_events = &mut self.events.end_block_events;

        let mut ctx = AContext {
            events: EventContext::new(end_block_events),
            stateful: &mut self.stateful,
            stateless: &mut self.stateless,
        };

        let result = self.module.end_block(&mut ctx, req).await;

        resp.consensus_param_updates = result.consensus_param_updates;
        resp.validator_updates = result.validator_updates;

        let events = mem::replace(end_block_events, Vec::new());

        resp.events = events;

        resp
    }

    async fn commit(&mut self) -> ResponseCommit {
        let mut resp = ResponseCommit::default();

        self.stateless
            .commit()
            .expect("Panic! commit failed when commit new block");
        self.stateful
            .commit()
            .expect("Panic! commit failed when commit new block");

        resp.data = self
            .stateful
            .root()
            .expect("Panic!, Can't read app hash when commit new block")
            .to_vec();

        resp
    }
}
