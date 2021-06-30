use crate::{Application, Transaction};

pub trait Module {
    type Transaction: Transaction;

    // type Storage;
    // type Event;
    // type RPC;
    type Application: Application<Self::Transaction>;

    fn name(&self) -> &str;

    fn version(&self) -> &str;

    fn get_application(&mut self) -> &Self::Application;
}
