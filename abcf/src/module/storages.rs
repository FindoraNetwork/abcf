use core::fmt::Debug;
use serde::{Deserialize, Serialize};

pub trait Value: Clone + Debug + Default + Serialize + for<'de> Deserialize<'de> {}

// Module declare
pub trait Storages {
    fn stateless_keys() -> &'static [&'static str];

    fn stateful_keys() -> &'static [&'static str];
}
