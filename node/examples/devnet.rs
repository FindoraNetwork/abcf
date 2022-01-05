#![feature(generic_associated_types)]

use std::convert::TryFrom;
use std::marker::PhantomData;

use abcf::module::types::{
    RequestBeginBlock, RequestCheckTx, RequestDeliverTx, ResponseCheckTx, ResponseDeliverTx,
};
/// Running in shell
///
/// ``` bash
/// $ cargo run --example devnet
/// ```
use abcf::{AppContext, Application, Event, RPCContext, RPCResponse, TxnContext};
use bs3::merkle::append_only::AppendOnlyMerkle;
use bs3::model::{Map, Value};
use bs3::MapStore;
use serde::{Deserialize, Serialize};
use sha3::Sha3_512;

/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct SendEvent {
    #[abcf(index)]
    pub_key: String,
    send_amount: Option<u64>,
}

#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
// #[dependence(external_module = "MockModule")]
pub struct MockModule {
    // /// In memory.
    pub inner: u32,
    #[stateful(merkle = "AppendOnlyMerkle")]
    pub sf_value: Value<u32>,
    #[stateless(merkle = "AppendOnlyMerkle")]
    pub sl_value: Value<u32>,
    #[stateless(merkle = "AppendOnlyMerkle")]
    pub sl_map: Map<i64, i64>,
}

#[abcf::rpcs]
impl MockModule {
    pub async fn get_owned_outputs(
        &mut self,
        _ctx: &mut RPCContext<'_, Self>,
        request: String,
    ) -> RPCResponse<String> {
        RPCResponse::new(request)
    }
}

pub mod call_rpc {
    include!(concat!(env!("OUT_DIR"), "/mockmodule.rs"));
}

/// Module's block logic.
#[abcf::application]
impl Application for MockModule {
    type Transaction = MockTransaction;

    async fn begin_block(&mut self, context: &mut AppContext<'_, Self>, _req: &RequestBeginBlock) {
        let height = &_req.header.as_ref().unwrap().height;
        let _ = context.stateless.sl_map.insert(*height, *height);

        if height > &1 {
            let key = height - 1;
            if let Ok(Some(_)) = context.stateless.sl_map.get(&key) {}
        }
    }

    async fn check_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        _req: &RequestCheckTx<Self::Transaction>,
    ) -> abcf::Result<ResponseCheckTx> {
        let e = SendEvent {
            pub_key: "123".to_string(),
            send_amount: Some(3),
        };
        context.events.emmit(e)?;

        Ok(Default::default())
    }

    async fn deliver_tx(
        &mut self,
        context: &mut TxnContext<'_, Self>,
        _req: &RequestDeliverTx<Self::Transaction>,
    ) -> abcf::Result<ResponseDeliverTx> {
        let e = SendEvent {
            pub_key: "123".to_string(),
            send_amount: Some(1),
        };
        context.events.emmit(e)?;

        Ok(Default::default())
    }
}

/// Module's methods.
#[abcf::methods]
impl MockModule {}

#[derive(Default)]
pub struct MockTransaction {}

#[derive(Default, Serialize, Deserialize)]
pub struct SimpleNodeTransaction {
    pub v: u64,
}

impl abcf::Transaction for SimpleNodeTransaction {}

impl abcf::module::FromBytes for SimpleNodeTransaction {
    fn from_bytes(bytes: &[u8]) -> abcf::Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_slice(bytes)?)
    }
}

impl TryFrom<&SimpleNodeTransaction> for MockTransaction {
    type Error = abcf::Error;

    fn try_from(_: &SimpleNodeTransaction) -> Result<Self, Self::Error> {
        Ok(MockTransaction {})
    }
}

#[abcf::manager(
    name = "simple_node",
    digest = "Sha3_512",
    version = 0,
    impl_version = "0.1.0",
    transaction = "SimpleNodeTransaction"
)]
pub struct SimpleManager {
    pub mock: MockModule,
    // #[dependence(external_module = "mock2")]
    pub mock2: MockModule,
}

fn main() {
    env_logger::init();
    use bs3::backend::MemoryBackend;

    let mock = MockModule::new(1);

    let mock2 = MockModule::new(2);

    let simple_node = SimpleManager::<MemoryBackend>::new(mock, mock2);

    let stateless = abcf::Stateless::<SimpleManager<MemoryBackend>> {
        mock: abcf::Stateless::<MockModule<MemoryBackend, Sha3_512>> {
            sl_map: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            sl_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
        mock2: abcf::Stateless::<MockModule<MemoryBackend, Sha3_512>> {
            sl_map: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            sl_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
    };

    let stateful = abcf::Stateful::<SimpleManager<MemoryBackend>> {
        mock: abcf::Stateful::<MockModule<MemoryBackend, Sha3_512>> {
            sf_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
        mock2: abcf::Stateful::<MockModule<MemoryBackend, Sha3_512>> {
            sf_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            __marker_s: PhantomData,
            __marker_d: PhantomData,
        },
    };

    let entry = abcf::entry::Node::new(stateless, stateful, simple_node);
    let node = abcf_node::Node::new(entry, "./target/abcf").unwrap();
    node.start().unwrap();
    std::thread::park();
}
