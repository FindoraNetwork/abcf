mod events;
pub use events::{EventContext, EventContextImpl};

mod call;
pub use call::CallContext;

pub struct Context<'a> {
    pub event: Option<EventContext<'a>>,
    // pub calls: CallContext<'a>,
}

pub struct RContext<'a, Sl, Sf> {
    // pub calls: CallContext<'a>,
    pub stateless: &'a mut Sl,
    pub stateful: &'a Sf,
}
