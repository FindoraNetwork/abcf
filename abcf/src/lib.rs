#![feature(generic_associated_types)]
#![no_std]

extern crate alloc;

pub mod module;
pub use module::{
    Application, Callable, Event, Genesis, Merkle, Module, ModuleMetadata, ModuleType, RPCResponse,
    RPCs, Storage, Transaction,
};

pub mod entry;

pub mod manager;
pub use manager::Context;

mod error;
pub use error::{Error, ModuleError, ModuleResult, Result};

pub use abcf_macros::*;

pub use async_trait::async_trait as application;

pub use tm_protos::abci;

pub use bs3;
pub use digest;

pub type Stateless<M> = <M as manager::ModuleStorage>::Stateless;
pub type Stateful<M> = <M as manager::ModuleStorage>::Stateful;

