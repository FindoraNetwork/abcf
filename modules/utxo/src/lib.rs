#![feature(generic_associated_types)]

use serde::{Serialize, Deserialize};
use abcf::{Event, Application};
use bs3::model::{Value, Map};

/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct Event1 {}

#[abcf::module(name = "utxo", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct UTXOModule {
    pub inner: u32,
    #[stateful]
    pub sf_value: Value<u32>,
    #[stateless]
    pub sl_value: Value<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

#[abcf::rpcs(module = "utxo")]
impl UTXOModule {}


/// Module's block logic.
#[abcf::application]
impl Application for UTXOModule {}

/// Module's methods.
#[abcf::methods]
impl UTXOModule {}
