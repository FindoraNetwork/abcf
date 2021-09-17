#![feature(generic_associated_types)]

use std::marker::PhantomData;

/// Running in shell
///
/// ``` bash
/// $ cargo run --example devnet
/// ```
use abcf::{module::StorageTransaction, Application, Event};
use bs3::model::{Map, Value};
use serde::{Deserialize, Serialize};
use sha3::Sha3_512;


/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct Event1 {}

#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct MockModule {
    // /// In memory.
    pub inner: u32,
    #[stateful]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}


#[abcf::rpcs]
impl MockModule {}

/// Module's block logic.
#[abcf::application]
impl Application for MockModule {
    type Transaction = MockTransaction;
}

/// Module's methods.
#[abcf::methods]
impl MockModule {}

pub struct MockTransaction {}

impl Default for MockTransaction {
    fn default() -> Self {
        MockTransaction {}
    }
}

#[derive(Serialize, Deserialize)]
pub struct SimpleNodeTransaction {
    pub v: u64,
}

impl abcf::Transaction for SimpleNodeTransaction {}

impl Default for SimpleNodeTransaction {
    fn default() -> Self {
        Self { v: 0 }
    }
}

impl abcf::module::FromBytes for SimpleNodeTransaction {
    fn from_bytes(bytes: &[u8]) -> abcf::Result<Self>
    where
        Self: Sized,
    {
        Ok(serde_json::from_slice(bytes)?)
    }
}

impl Into<MockTransaction> for SimpleNodeTransaction {
    fn into(self) -> MockTransaction {
        MockTransaction {}
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
    pub mock2: MockModule,
}


fn main() {
    env_logger::init();
    use bs3::backend::MemoryBackend;

    let mock = MockModule::new(1);

    let mock2 = MockModule::new(2);

    let simple_node = SimpleManager::<MemoryBackend> {
        mock,
        mock2,
        __marker_s: PhantomData,
    };

    let stateless = abcf::Stateless::<SimpleManager<MemoryBackend>> {
        mock: abcf::Stateless::<MockModule<MemoryBackend>> {
            sl_map: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            sl_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            __marker_s:PhantomData,
        },
        mock2: abcf::Stateless::<MockModule<MemoryBackend>> {
            sl_map: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            sl_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            __marker_s:PhantomData,
        },
    };

    let stateful = abcf::Stateful::<SimpleManager<MemoryBackend>> {
        mock: abcf::Stateful::<MockModule<MemoryBackend>> {
            sf_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            __marker_s:PhantomData,
        },
        mock2: abcf::Stateful::<MockModule<MemoryBackend>> {
            sf_value: abcf::bs3::SnapshotableStorage::new(Default::default(), MemoryBackend::new())
                .unwrap(),
            __marker_s:PhantomData,
        },
    };

    let entry = abcf::entry::Node::new(stateless, stateful, simple_node);
    let node = abcf_node::Node::new(entry, "./target/abcf").unwrap();
    node.start().unwrap();
    std::thread::park();
}
