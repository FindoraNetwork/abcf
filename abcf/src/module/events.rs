use crate::Result;
use alloc::string::String;
use core::fmt::Debug;
use tm_protos::abci;

/// Define event of module.
pub trait Event: Debug {
    /// Get current event name.
    fn name(&self) -> &str;

    /// Build this event to abci event.
    fn to_abci_event(&self) -> Result<abci::Event>;

    fn from_abci_event(&mut self, e: abci::Event) -> Result<()>;

    fn from_abci_event_string(&mut self, str: String) -> Result<()>;
}

/// Define event attributes.
pub trait EventAttr {
    /// Build event attributes.
    fn to_abci_event(&self) -> abci::EventAttribute;
}
