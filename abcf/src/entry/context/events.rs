use crate::{module::Event, Result};
use alloc::vec::Vec;
use tm_protos::abci;

pub struct EventContext<'a> {
    pub events: &'a mut Vec<abci::Event>,
}

impl<'a> EventContext<'a> {
    pub fn new(events: &'a mut Vec<abci::Event>) -> Self {
        EventContext { events }
    }

    pub fn emmit(&mut self, event: impl Event) -> Result<()> {
        let event = event.to_abci_event()?;
        self.events.push(event);
        Ok(())
    }
}

#[derive(Default)]
pub struct EventContextImpl {
    pub begin_block_events: Vec<abci::Event>,
    pub check_tx_events: Vec<abci::Event>,
    pub deliver_tx_events: Vec<abci::Event>,
    pub end_block_events: Vec<abci::Event>,
}
