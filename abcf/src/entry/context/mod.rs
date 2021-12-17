mod events;

pub use events::{EventContext, EventContextImpl};

use crate::{manager::ModuleStorage, Stateful, StatefulBatch, Stateless, StatelessBatch};

pub struct RContext<'a, M: ModuleStorage> {
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a Stateful<M>,
}

pub struct AContext<'a, M: ModuleStorage> {
    pub events: EventContext<'a>,
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a mut Stateful<M>,
}

pub struct TContext<'a, M: ModuleStorage>
where
    Self: 'a,
{
    pub events: EventContext<'a>,
    pub stateless: StatelessBatch<'a, M>,
    pub stateful: StatefulBatch<'a, M>,
}
