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
    pub sl_map: Map<i32, u32>,
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
