#![no_std]

extern crate alloc;

mod module;
pub use module::{
    Application, Event, Genesis, Module, ModuleMetadata, RPCResponse, RPCs, Storage, Transaction,
};

pub mod abci;
pub use abci::Node;

mod error;
pub use error::{Error, Result, ModuleError, ModuleResult};
