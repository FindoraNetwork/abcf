use core::fmt::Debug;
// use alloc::{string::ToString, vec::Vec};
use tm_protos::abci;

/// Define event of module.
pub trait Event: Debug {
    /// Get current event name.
    fn name(&self) -> &str;

    /// Build this event to abci event.
    fn to_abci_event(&self) -> abci::Event;
//     fn to_abci_event(&self, attrs: &[&dyn EventAttr]) -> abci::Event {
        // let mut attributes = Vec::new();
        // for attr in attrs {
        //     let abci_attr = attr.to_abci_event();
        //     attributes.push(abci_attr);
        // }
        // abci::Event {
        //     r#type: self.name().to_string(),
        //     attributes,
        // }
    // }

    /// Get list of events.
    fn all() -> &'static [&'static str];
}

/// Define event attributes.
pub trait EventAttr {
    /// Build event attributes.
    fn to_abci_event(&self) -> abci::EventAttribute;
}

