#![no_std]

extern crate alloc;

mod module;
pub use module::Module;

mod application;
pub use application::Application;

pub mod message;

mod transaction;
pub use transaction::Transaction;

pub enum Error {}

pub type Result<T> = core::result::Result<T, Error>;
