#![feature(generic_associated_types)]

/// Running in shell
///
/// ``` bash
/// $ cargo run --example devnet
/// ```
use abcf::{Application, Event};
use bs3::model::{Map, Value};
use serde::{Deserialize, Serialize};

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
    type Transaction = Vec<u8>;
}

/// Module's methods.
#[abcf::methods]
impl MockModule {}

pub struct SimpleNode<S: abcf::bs3::Store + 'static> {
    pub mock: MockModule<S>,
    pub mock2: MockModule<S>,
}

impl<S> abcf::Module for SimpleNode<S>
where
    S: abcf::bs3::Store + 'static
{
    fn metadata(&self) -> abcf::ModuleMetadata<'_> {
        abcf::ModuleMetadata {
            name: "simple_node",
            module_type: abcf::ModuleType::Manager,
            version: 1,
            impl_version: "0.1",
            genesis: abcf::Genesis { target_height: 0 },
        }
    }
}

pub struct SimpleNodeSl<S: abcf::bs3::Store + 'static> {
    pub mock: abcf::Stateless<MockModule<S>>,
    pub mock2: abcf::Stateless<MockModule<S>>,
}

pub struct SimpleNodeSf<S: abcf::bs3::Store + 'static> {
    pub mock: abcf::Stateful<MockModule<S>>,
    pub mock2: abcf::Stateful<MockModule<S>>,
}

// #[async_trait::async_trait]
// impl<S> abcf::entry::Application<SimpleNodeSf<S>, SimpleNodeSl<S>> for SimpleNode<S>
// where
//     S: abcf::bs3::Store + 'static
// {
//     /// Define how to check transaction.
//     ///
//     /// In this function, do some lightweight check for transaction, for example: check signature,
//     /// check balance and so on.
//     /// This method will be called at external user or another node.
//     async fn check_tx(
//         &mut self,
//         context: &mut abcf::entry::TContext<StatelessTx<'_>, StatefulTx<'_>>,
//         _req: abcf::module::types::RequestCheckTx,
//     ) -> abcf::ModuleResult<abcf::module::types::ResponseCheckTx> {
//         let mut ctx = abcf::manager::TContext {
//             events: abcf::entry::EventContext {
//                 events: context.events.events,
//             },
//             stateful: context.stateful,
//             stateless: context.stateless,
//         };
//
//         let result = self
//             .mock
//             .check_tx(&mut ctx, &_req)
//             .await
//             .map_err(|e| ModuleError {
//                 namespace: String::from("mock"),
//                 error: e,
//             })?;
//
//         Ok(result)
//     }
//
//     /// Begin block.
//     async fn begin_block(
//         &mut self,
//         context: &mut abcf::entry::AContext<EmptyStorage, EmptyStorage>,
//         _req: abcf::module::types::RequestBeginBlock,
//     ) {
//         let mut ctx = abcf::manager::AContext {
//             events: abcf::entry::EventContext {
//                 events: context.events.events,
//             },
//             stateful: context.stateful,
//             stateless: context.stateless,
//         };
//         self.mock.begin_block(&mut ctx, &_req).await;
//     }
//
//     /// Execute transaction on state.
//     async fn deliver_tx(
//         &mut self,
//         context: &mut abcf::entry::TContext<
//             <EmptyStorage as StorageTransaction>::Transaction<'_>,
//             <EmptyStorage as StorageTransaction>::Transaction<'_>,
//         >,
//         _req: abcf::module::types::RequestDeliverTx,
//     ) -> abcf::ModuleResult<abcf::module::types::ResponseDeliverTx> {
//         let mut ctx = abcf::manager::TContext {
//             events: abcf::entry::EventContext {
//                 events: context.events.events,
//             },
//             stateful: context.stateful,
//             stateless: context.stateless,
//         };
//
//         let result = self
//             .mock
//             .deliver_tx(&mut ctx, &_req)
//             .await
//             .map_err(|e| ModuleError {
//                 namespace: String::from("mock"),
//                 error: e,
//             })?;
//
//         Ok(result)
//     }
//
//     /// End Block.
//     async fn end_block(
//         &mut self,
//         context: &mut abcf::entry::AContext<EmptyStorage, EmptyStorage>,
//         _req: abcf::module::types::RequestEndBlock,
//     ) -> abcf::module::types::ResponseEndBlock {
//         let mut ctx = abcf::manager::AContext {
//             events: abcf::entry::EventContext {
//                 events: context.events.events,
//             },
//             stateful: context.stateful,
//             stateless: context.stateless,
//         };
//         self.mock.end_block(&mut ctx, &_req).await
//     }
// }

#[async_trait::async_trait]
impl<S> abcf::entry::RPCs<SimpleNodeSl<S>, SimpleNodeSf<S>> for SimpleNode<S>
where
    S: abcf::bs3::Store + 'static
{
    async fn call(
        &mut self,
        ctx: &mut abcf::entry::RContext<SimpleNodeSl<S>, SimpleNodeSf<S>>,
        method: &str,
        params: serde_json::Value,
    ) -> abcf::ModuleResult<Option<serde_json::Value>> {
        use abcf::RPCs;
        let mut paths = method.split("/");
        let module_name = paths.next().ok_or(abcf::ModuleError {
            namespace: String::from("abcf.manager"),
            error: abcf::Error::QueryPathFormatError,
        })?;

        let method = paths.next().ok_or(abcf::ModuleError {
            namespace: String::from("abcf.managing"),
            error: abcf::Error::QueryPathFormatError,
        })?;

        match module_name {
            "mock" => {
                let mut context = abcf::manager::RContext {
                    stateful: &ctx.stateful.mock,
                    stateless: &mut ctx.stateless.mock,
                };

                self
                .mock
                .call(&mut context, method, params)
                .await
                .map_err(|e| abcf::ModuleError {
                    namespace: String::from("mock"),
                    error: e,
                })
            }
            _ => Err(abcf::ModuleError {
                namespace: String::from("abcf.manager"),
                error: abcf::Error::NoModule,
            }),
        }
    }
}

fn main() {
    env_logger::init();
    use bs3::backend::MemoryBackend;

    let mock = MockModule::new(1);

    let mock2 = MockModule::new(2);
    //
    let simple_node = SimpleNode::<MemoryBackend> { mock, mock2 };
    //
    // let entry = abcf::entry::Node::new(EmptyStorage::new(), EmptyStorage::new(), simple_node);
    // let node = abcf_node::Node::new(entry, "./target/abcf").unwrap();
    // node.start().unwrap();
//     std::thread::park();
}
