#![feature(generic_associated_types)]

use std::marker::PhantomData;

/// Running in shell
///
/// ``` bash
/// $ cargo run --example devnet
/// ```
use abcf::{
    entry::Tree, module::StorageTransaction, Application, Event, Merkle,
    Storage,
};
use bs3::model::{Map, Value};

/// Module's Event
#[derive(Debug, Event)]
pub struct Event1 {}

#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct MockModule<S, D>
where
    S: abcf::bs3::Store,
    D: digest::Digest,
{
    // /// In memory.
    pub inner: u32,
    pub maker_s: PhantomData<S>,
    pub maker_d: PhantomData<D>,
    #[stateful]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

#[abcf::rpcs]
impl<S, D> MockModule<S, D> 
where
    S: abcf::bs3::Store,
    D: digest::Digest,
{}

/// Module's block logic.
#[abcf::application]
impl<S, D> Application<abcf::Stateless<Self>, abcf::Stateful<Self>> for MockModule<S, D>
where
    S: abcf::bs3::Store + 'static,
    D: digest::Digest + Send + Sync,
{

}

/// Module's methods.
impl<S, D> MockModule<S, D>
where
    S: abcf::bs3::Store,
    D: digest::Digest,
{
}

pub struct SimpleNode {
    pub mock: MockModule<bs3::backend::MemoryBackend, sha3::Sha3_512>,
}

impl abcf::Module for SimpleNode {
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

// type StatelessTx<'a> = <EmptyStorage as StorageTransaction>::Transaction<'a>;
// type StatefulTx<'a> = <EmptyStorage as StorageTransaction>::Transaction<'a>;
//
// #[abcf::application]
// impl abcf::entry::Application<EmptyStorage, EmptyStorage> for SimpleNode {
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
//
// #[abcf::application]
// impl abcf::entry::RPCs<EmptyStorage, EmptyStorage> for SimpleNode {
//     async fn call(
//         &mut self,
//         ctx: &mut abcf::entry::RContext<EmptyStorage, EmptyStorage>,
//         method: &str,
//         params: serde_json::Value,
//     ) -> abcf::ModuleResult<Option<serde_json::Value>> {
//         use abcf::RPCs;
//         let mut paths = method.split("/");
//         let module_name = paths.next().ok_or(ModuleError {
//             namespace: String::from("abcf.manager"),
//             error: Error::QueryPathFormatError,
//         })?;
//
//         let method = paths.next().ok_or(ModuleError {
//             namespace: String::from("abcf.managing"),
//             error: Error::QueryPathFormatError,
//         })?;
//
//         let mut context = abcf::manager::RContext {
//             stateful: ctx.stateful,
//             stateless: ctx.stateless,
//         };
//
//         match module_name {
//             "mock" => self
//                 .mock
//                 .call(&mut context, method, params)
//                 .await
//                 .map_err(|e| ModuleError {
//                     namespace: String::from("mock"),
//                     error: e,
//                 }),
//             _ => Err(ModuleError {
//                 namespace: String::from("abcf.manager"),
//                 error: Error::NoModule,
//             }),
//         }
//     }
// }
//
fn main() {
    //     env_logger::init();
    // let mock = MockModule { inner: 0 };
    //
    // let simple_node = SimpleNode { mock };
    //
    // let entry = abcf::entry::Node::new(EmptyStorage::new(), EmptyStorage::new(), simple_node);
    // let node = abcf_node::Node::new(entry, "./target/abcf").unwrap();
    // node.start().unwrap();
    //     std::thread::park();
}
