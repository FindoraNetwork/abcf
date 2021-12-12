mod node;
pub use node::Node;

mod context;
pub use context::{AContext, EventContext, EventContextImpl, RContext, TContext};

mod prelude;
pub use prelude::{Application, RPCs, Tree};

use crate::{Stateful, StatefulBatch, Stateless, StatelessBatch};

pub type AppContext<'a, M> = AContext<'a, Stateless<M>, Stateful<M>>;
pub type TxnContext<'a, M> = TContext<'a, StatelessBatch<'a, M>, StatefulBatch<'a, M>>;
pub type RPCContext<'a, M> = RContext<'a, Stateless<M>, Stateful<M>>;
