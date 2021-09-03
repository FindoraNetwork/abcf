mod events;

use bs3::Store;
pub use events::{EventContext, EventContextImpl};

use crate::Storage;

pub struct AContext<'a, Sl, Sf> {
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}

pub struct RContext<'a, Sl, Sf> {
    pub stateless: &'a mut Sl,
    pub stateful: &'a Sf,
}

pub struct DContext<'a, S, Sl, Sf>
where
    S: Store,
    Sl: Storage<S>,
    Sf: Storage<S>,
{
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl::Transaction,
    pub stateful: &'a mut Sf::Transaction,
}

