use crate::entry::EventContext;

mod call;
pub use call::{CallContext, CallEntry, CallImpl, CallEntryTrait};

pub struct Context<'a> {
    pub event: Option<EventContext<'a>>,
    // pub calls: CallContext<'a>,
}

pub struct RContext<'a, Sl, Sf> {
    // pub calls: &'a dyn Any,
    pub stateless: &'a mut Sl,
    pub stateful: &'a Sf,
}

pub struct AContext<'a, Sl, Sf> {
    // pub calls: CallContext<'a>,
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}

pub struct TContext<'a, Sl, Sf> {
    pub calls: CallContext<'a>,
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}
