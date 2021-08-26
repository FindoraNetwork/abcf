mod events;
pub use events::{EventContext, EventContextImpl};

pub mod storage;
pub use storage::StorageContext;

pub struct Context<'a> {
    pub event: Option<EventContext<'a>>,
    pub storage: StorageContext,
}
