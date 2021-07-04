use crate::server::ABCIApplication;
use crate::ABCIMemServer;
use crate::{rpc, MemClient, MemServer};
use std::slice;
use ffi_support::ByteBuffer;

static mut MEM_SERVER: Option<MemServer<ABCIMemServer>> = None;
static mut MEM_CLIENT: Option<MemClient> = None;

#[no_mangle]
pub extern "C" fn call(req_ptr: *const u8, req_len: usize) -> ByteBuffer {
    let req_bytes = unsafe {
        slice::from_raw_parts(req_ptr, req_len)
    };
    let resp = unsafe {
        MEM_CLIENT.as_mut().unwrap().call(req_bytes)
    };
    resp.into()
}

#[no_mangle]
pub extern "C" fn start() {
    env_logger::init();
    let (client, server) = rpc(100000, ABCIMemServer::new(ABCIApplication::default()));
    println!("init!");
    unsafe {
        MEM_SERVER = Some(server);
        MEM_CLIENT = Some(client);
        MEM_SERVER.as_mut().unwrap().start();
    }
}
