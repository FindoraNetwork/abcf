mod events;

pub use events::{EventContext, EventContextImpl};

pub struct RContext<'a, Sl, Sf> {
    pub stateless: &'a mut Sl,
    pub stateful: &'a Sf,
}

pub struct AContext<'a, Sl, Sf> {
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}

pub struct TContext<'a, Sl, Sf> {
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}
