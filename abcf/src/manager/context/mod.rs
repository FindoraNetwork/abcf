mod events;
pub use events::{EventContext, EventContextImpl};

pub mod storage;
pub use storage::StorageContext;

mod call;
pub use call::CallContext;

pub struct Context<'a> {
    pub event: Option<EventContext<'a>>,
    pub storage: StorageContext,
    pub calls: CallContext<'a>,
}
