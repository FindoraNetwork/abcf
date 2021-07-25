mod events;
pub use events::{EventContext, EventContextImpl};

pub struct Context<'a> {
    pub event: Option<EventContext<'a>>,
}
