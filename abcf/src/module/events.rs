use core::fmt::Debug;
use crate::Result;
use tm_protos::abci;

/// Define event of module.
pub trait Event: Debug {
    /// Get current event name.
    fn name(&self) -> &str;

    /// Build this event to abci event.
    fn to_abci_event(&self) -> Result<abci::Event>;
}

/// Define event attributes.
pub trait EventAttr {
    /// Build event attributes.
    fn to_abci_event(&self) -> abci::EventAttribute;
}
