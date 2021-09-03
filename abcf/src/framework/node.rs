use core::marker::PhantomData;

use alloc::{boxed::Box, string::String};

use bs3::Store;
use digest::Digest;
use tm_protos::abci::{RequestInfo, RequestInitChain, ResponseInfo, ResponseInitChain};

use crate::{Application, Merkle, Module, RPCs, Storage};

pub struct Node<S, D, Sl, Sf, M>
where
    S: Store,
    D: Digest,
    Sl: Storage<S>,
    Sf: Storage<S> + Merkle<D>,
    M: Module + Application + RPCs,
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
    M: Module + Application + RPCs,
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
}

#[async_trait::async_trait]
impl<S, D, Sl, Sf, M> tm_abci::Application for Node<S, D, Sl, Sf, M>
where
    S: Store,
    D: Digest + Send,
    Sl: Storage<S>,
    Sf: Storage<S> + Merkle<D>,
    M: Module + Application + RPCs,
{
    async fn init_chain(
        &mut self,
        _request: RequestInitChain,
    ) -> ResponseInitChain {
        let mut resp = ResponseInitChain::default();

        // if failed, safety exit.
        resp.app_hash = self.stateful.root().expect("get app hash failed").to_vec();

        resp
    }

    async fn info(&mut self, _request: RequestInfo) -> ResponseInfo {
        let mut resp = ResponseInfo::default();

        resp.version = String::from(self.module.metadata().impl_version);
        resp.app_version = self.module.metadata().version;

        // compare height.
        let stateful_height = self.stateful.height().expect("get current height for stateful storage failed");
        let stateless_height = self.stateless.height().expect("get current height for stateless storage failed");

        if stateless_height >= stateful_height {
            self.stateless.rollback(stateful_height).expect("rollback to height failed.");
            resp.last_block_height = stateful_height;
            resp.last_block_app_hash = self.stateful.root().expect("get app hash failed").to_vec();
        } else {
            self.stateful.rollback(stateless_height).expect("rollback to height failed.");
            resp.last_block_height = stateful_height;
            resp.last_block_app_hash = self.stateful.root().expect("get app hash failed").to_vec();
        }

        resp
    }
}
