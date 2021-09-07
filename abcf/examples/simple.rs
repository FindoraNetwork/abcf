#![feature(generic_associated_types)]

use abcf::{Application, Event};
use bs3::model::{Map, Value};

/// Module's Event
#[derive(Debug, Event)]
pub struct Event1 {}

#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct MockModule {
    /// In memory.
    pub inner: u32,
    #[stateful]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

mod __abcf_storage_mockmodule {
    use super::*;
    use abcf::Result;
    pub struct ABCFModuleMockModuleSl<S>
    where
        S: abcf::bs3::Store,
    {
        pub sl_value: abcf::bs3::SnapshotableStorage<S, Value<u32>>,
        pub sl_map: abcf::bs3::SnapshotableStorage<S, Map<i32, u32>>,
    }
    pub struct ABCFModuleMockModuleSlTx<'a, S>
    where
        S: abcf::bs3::Store,
    {
        pub sl_value: abcf::bs3::Transaction<'a, S, Value<u32>>,
        pub sl_map: abcf::bs3::Transaction<'a, S, Map<i32, u32>>,
    }
    impl<S> abcf::Storage for ABCFModuleMockModuleSl<S>
    where
        S: abcf::bs3::Store,
    {
        fn rollback(&mut self, height: i64) -> Result<()> {
            self.sl_value.rollback(height)?;
            self.sl_map.rollback(height)?;
            Ok(())
        }
        fn height(&self) -> Result<i64> {
            Ok(0)
        }
        fn commit(&mut self) -> Result<()> {
            self.sl_value.commit()?;
            self.sl_map.commit()?;
            Ok(())
        }
    }
    pub struct ABCFModuleMockModuleSf<S>
    where
        S: abcf::bs3::Store,
    {
        pub sf_value: abcf::bs3::SnapshotableStorage<S, Value<u32>>,
    }
}
/// Module's rpc.
#[abcf::rpcs]
impl MockModule {}

/// Module's block logic.
#[abcf::application]
impl Application for MockModule {}

/// Module's methods.
impl MockModule {}

// pub struct SimpleNodeSl {}
//
// pub struct SimpleNodeSf {}
//
// pub struct SimpleNode {
//     mock: MockModule,
// }
//
// impl Module for SimpleNode {
//     fn metadata(&self) -> ModuleMetadata<'_> {
//         ModuleMetadata {
//             name: "simple_node",
//             module_type: abcf::ModuleType::Manager,
//             version: 1,
//             impl_version: "0.1",
//             genesis: Genesis { target_height: 0 },
//         }
//     }
// }

fn main() {}
