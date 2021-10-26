#![feature(generic_associated_types)]
#![no_std]

extern crate alloc;

pub mod module;
use core::fmt::Debug;

pub use module::{
    Application, Callable, Event, FromBytes, Genesis, Merkle, Module, ModuleMetadata, ModuleType,
    RPCResponse, RPCs, Storage, ToBytes, Transaction,
};

pub mod entry;

pub mod manager;
pub use manager::Context;

mod error;
pub use error::{Error, ModuleError, ModuleResult, Result};

pub use abcf_macros::*;

pub use tm_protos;

pub use bs3;
pub use digest;
pub use log;

pub type Stateless<M> = <M as manager::ModuleStorage>::Stateless;
pub type Stateful<M> = <M as manager::ModuleStorage>::Stateful;

pub type StatelessBatch<'a, M> = <Stateless<M> as module::StorageTransaction>::Transaction<'a>;
pub type StatefulBatch<'a, M> = <Stateful<M> as module::StorageTransaction>::Transaction<'a>;

pub type StatelessCache<M> = <Stateless<M> as module::StorageTransaction>::Cache;
pub type StatefulCache<M> = <Stateful<M> as module::StorageTransaction>::Cache;

pub trait Config: Send + Sync + Debug + Clone + 'static {}
