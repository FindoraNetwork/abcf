use crate::{Application, Transaction};

pub struct ModuelAttributes {}

pub trait Module {
    type Transaction: Transaction + Send + 'static;

    // type Storage;
    // type Event;
    // type RPC;
    type Application: Application<Self::Transaction>;

    fn name(&self) -> &str;

    fn version(&self) -> &str;

    fn application(&mut self) -> &mut Self::Application;

    fn config(&self) -> ModuelAttributes;
}
