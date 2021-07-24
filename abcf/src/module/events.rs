use core::fmt::Debug;
use serde::{Deserialize, Serialize};

pub trait Event: Clone + Debug + Default + Serialize + for<'de> Deserialize<'de> {
    fn name(&self) -> &str;
}

pub trait Events {
    fn all_events() -> &'static [&'static str];
}
