#![no_std]

extern crate alloc;

pub mod module;

pub mod abci;

mod error;
pub use error::{Error, Result};
