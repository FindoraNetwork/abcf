use crate::Config;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TxOutput<C: Config> {
    pub code: C::AssetCode,
    pub amount: u64,
    pub owner: C::Address,
}
