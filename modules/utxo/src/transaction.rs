use crate::{data::TxOutput, Config};

pub struct TransactionBody<C: Config> {
    pub inputs: Vec<C::OutputId>,
    pub output: Vec<TxOutput<C>>,
}

pub struct Transaction<C: Config> {
    pub body: TransactionBody<C>,
    pub signature: Option<C::Signature>,
}

impl<C: Config> Default for Transaction<C> {
    fn default() -> Self {
        Self {
            signature: None,
            body: TransactionBody {
                inputs: Vec::new(),
                output: Vec::new(),
            },
        }
    }
}
