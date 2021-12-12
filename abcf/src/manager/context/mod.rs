use crate::entry::EventContext;

mod call;
pub use call::{CallContext, CallEntry, CallImpl};

pub struct Context<'a> {
    pub event: Option<EventContext<'a>>,
    // pub calls: CallContext<'a>,
}

pub struct Dependence<'a, M, Sl, Sf> {
    pub module: &'a mut M,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}

pub struct RContext<'a, Sl, Sf, D> {
    pub stateless: &'a mut Sl,
    pub stateful: &'a Sf,
    pub deps: D,
}

pub struct AContext<'a, Sl, Sf, D> {
    pub calls: CallContext<'a>,
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
    pub deps: D,
}

pub struct TContext<'a, Sl, Sf, D> {
    pub calls: CallContext<'a>,
    pub events: EventContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
    pub deps: D,
}
