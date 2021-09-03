mod events;

pub use events::{EventContext, EventContextImpl};

pub struct Context<'a, Sl, Sf> {
    pub events: Option<&'a EventContext<'a>>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}

pub struct RPCContext<'a, Sl, Sf> {
    pub events: Option<&'a EventContext<'a>>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a Sf,
}
