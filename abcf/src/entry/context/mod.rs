mod events;

use bs3::Store;
pub use events::{EventContext, EventContextImpl};

use crate::{Storage, module::StorageTransaction};

pub struct AContext<'a, Sl, Sf> {
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}

pub struct RContext<'a, Sl, Sf> {
    pub stateless: &'a mut Sl,
    pub stateful: &'a Sf,
}

pub struct TContext<'a, Sl, Sf>
where
    Sl: StorageTransaction,
    Sf: StorageTransaction,
{
    pub events: EventContext<'a>,
    pub stateless: Sl::Transaction,
    pub stateful: Sf::Transaction,
}
