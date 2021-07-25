use core::fmt::Debug;
use serde::{Deserialize, Serialize};

/// Define event of module.
pub trait Event: Clone + Debug + Default + Serialize + for<'de> Deserialize<'de> {
    fn name(&self) -> &str;
}
