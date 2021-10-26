use crate::Result;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt::Debug;
use serde_json::Value;
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

pub trait EventValue {
    fn to_value_bytes(&self) -> Result<Vec<u8>>;
}

impl<T: serde::Serialize> EventValue for T {
    fn to_value_bytes(&self) -> Result<Vec<u8>> {
        let v = serde_json::to_value(self)?;
        Ok(match v {
            Value::Null => Vec::new(),
            Value::String(s) => s.as_bytes().to_vec(),
            _ => serde_json::to_vec(&v)?,
        })
    }
}
