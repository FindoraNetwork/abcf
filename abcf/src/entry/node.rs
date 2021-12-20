use core::{marker::PhantomData, mem};

use alloc::{boxed::Box, string::String, vec::Vec};

use digest::Digest;
use tm_protos::abci::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, RequestEndBlock, RequestInfo,
    RequestInitChain, RequestQuery, ResponseBeginBlock, ResponseCheckTx, ResponseCommit,
    ResponseDeliverTx, ResponseEndBlock, ResponseInfo, ResponseInitChain, ResponseQuery,
};

use crate::{
    module::StorageTransaction, Error, Merkle, Module, ModuleError, ModuleResult, Stateful,
    Stateless, Storage,
};

use super::{
    context::TContext,
    prelude::{Application, RPCs, Tree},
    AContext, EventContext, EventContextImpl, RContext,
};

pub struct Node<D, M>
where
    D: Digest,
    M: Module + Application + RPCs,
{
    stateless: Stateless<M>,
    stateful: Stateful<M>,
    marker_d: PhantomData<D>,
    module: M,
    events: EventContextImpl,
}

impl<D, M> Node<D, M>
where
    D: Digest,
    M: Module + Application + RPCs,
{
    pub fn new(stateless: Stateless<M>, stateful: Stateful<M>, module: M) -> Self {
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
        &mut self,
        store: &str,
        sub_path: Option<&str>,
        height: i64,
    ) -> ModuleResult<(Vec<u8>, Vec<u8>)> {
        let key = sub_path.ok_or(ModuleError {
            namespace: String::from("abcf.store"),
            error: Error::QueryPathFormatError,
        })?;
        // let value = self.stateless.get(key, height)?;
        let value = match store {
            "stateless" => self.stateless.get(key, height)?,
            "stateful" => self.stateful.get(key, height)?,
            _ => Vec::new(),
        };
        Ok((key.as_bytes().to_vec(), value))
    }

    async fn _check_tx(&mut self, req: RequestCheckTx) -> ResponseCheckTx {
        let result = {
            let check_tx_events = &mut self.events.check_tx_events;

            let stateful_tx = self.stateful.transaction();
            let stateless_tx = self.stateless.transaction();

            let mut ctx = TContext {
                events: EventContext::new(check_tx_events),
                stateless: stateless_tx,
                stateful: stateful_tx,
            };

            self.module.check_tx(&mut ctx, req).await
        };

        let mut resp = ResponseCheckTx::default();

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

    async fn _deliver_tx(&mut self, req: RequestDeliverTx) -> ResponseDeliverTx {
        let mut resp = ResponseDeliverTx::default();

        let (result, sf_cache, sl_cache) = {
            let deliver_tx_events = &mut self.events.deliver_tx_events;

            let stateful_tx = self.stateful.transaction();
            let stateless_tx = self.stateless.transaction();

            let mut ctx = TContext {
                events: EventContext::new(deliver_tx_events),
                stateless: stateless_tx,
                stateful: stateful_tx,
            };

            let result = self.module.deliver_tx(&mut ctx, req).await;

            let stateful_cache = Stateful::<M>::cache(ctx.stateful);
            let stateless_cache = Stateless::<M>::cache(ctx.stateless);
            (result, stateful_cache, stateless_cache)
        };

        match result {
            Ok(v) => {
                resp.data = v.data;
                resp.gas_wanted = v.gas_wanted;
                resp.gas_used = v.gas_used;

                self.stateful.execute(sf_cache);
                self.stateless.execute(sl_cache);
            }
            Err(e) => {
                resp.code = e.error.code();
                resp.log = e.error.message();
                resp.codespace = e.namespace;
            }
        }

        let events = mem::replace(&mut self.events.deliver_tx_events, Vec::new());

        resp.events = events;

        resp
    }
}

#[async_trait::async_trait]
impl<D, M> tm_abci::Application for Node<D, M>
where
    D: Digest + Send + Sync,
    Stateful<M>: Merkle<D> + 'static,
    Stateless<M>: 'static,
    M: Module + Application + RPCs,
{
    async fn init_chain(&mut self, _request: RequestInitChain) -> ResponseInitChain {
        let mut resp = ResponseInitChain::default();

        resp.app_hash = self.stateful.root().expect("get app hash failed").to_vec();

        self.stateful
            .commit()
            .expect("init chain commit error on stateful");

        self.stateless
            .commit()
            .expect("init chain commit error on stateless");

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

        let mut target_height = if stateless_height >= stateful_height {
            stateful_height - 1
        } else {
            stateless_height - 1
        };

        if target_height < 0 {
            target_height = 0
        }

        if target_height > stateful_height {
            self.stateful
                .rollback(target_height)
                .expect("rollback to height failed.");
        }

        if target_height > stateless_height {
            self.stateless
                .rollback(target_height)
                .expect("rollback to height failed.");
        }

        resp.last_block_height = target_height;
        resp.last_block_app_hash = self.stateful.root().expect("get app hash failed").to_vec();

        resp
    }

    async fn query(&mut self, request: RequestQuery) -> ResponseQuery {
        let mut paths = request.path.splitn(2, "/");

        let mut resp = ResponseQuery::default();

        let target_path = paths.next();

        let sub_path = paths.next();

        let res = match target_path {
            Some("rpc") => self.call_rpc(sub_path, request.data.as_ref()).await,
            Some("stateful") => self.call_store("stateful", sub_path, request.height).await,
            Some("stateless") => self.call_store("stateless", sub_path, request.height).await,
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
        self._check_tx(req).await
    }

    async fn begin_block(&mut self, req: RequestBeginBlock) -> ResponseBeginBlock {
        let mut resp = ResponseBeginBlock::default();

        let begin_block_events = &mut self.events.begin_block_events;

        if let Some(header) = &req.header {
            if header.height - 1
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
            if header.height - 1
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
        self._deliver_tx(req).await
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
