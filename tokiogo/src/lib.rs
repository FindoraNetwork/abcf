#![feature(vec_into_raw_parts)]

mod memrpc;
pub use memrpc::{rpc, MemClient, MemServer, Server};

mod c_api;

mod server;
pub use server::ABCIMemServer;
