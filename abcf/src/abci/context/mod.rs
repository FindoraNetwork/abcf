mod events;
pub use events::EventContext;

pub struct Context<'a> {
    pub event: EventContext<'a>,
}
