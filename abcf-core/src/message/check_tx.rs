use crate::message::event::Event;
use crate::Transaction;

pub enum CheckTxType {
    New,
    Recheck,
}

impl Default for CheckTxType {
    fn default() -> Self {
        CheckTxType::New
    }
}

#[derive(Default)]
pub struct Request<T: Transaction> {
    pub tx: T,
    pub t: CheckTxType,
}

#[derive(Default)]
pub struct Response {
    pub code: u32,
    pub data: Vec<u8>,
    pub log: String,
    pub info: String,
    pub gas_wanted: i64,
    pub gas_used: i64,
    pub events: Vec<Event>,
    pub codespace: String,
    pub sender: String,
    pub priority: i64,
}
