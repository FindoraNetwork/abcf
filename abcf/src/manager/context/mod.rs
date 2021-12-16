use crate::{entry::EventContext, Stateful, StatefulBatch, Stateless, StatelessBatch};

use super::ModuleStorage;

pub trait Dependence {
    type RPC<'a>: Send
    where
        Self: 'a;

    type App<'a>: Send
    where
        Self: 'a;

    type Txn<'a>: Send
    where
        Self: 'a;
}

pub type AppDependence<'a, M> = <M as Dependence>::RPC<'a>;

pub struct RDependence<'a, M: ModuleStorage> {
    pub module: &'a mut M,
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a Stateful<M>,
}

pub struct ADependence<'a, M: ModuleStorage> {
    pub module: &'a mut M,
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a mut Stateful<M>,
}

pub struct TDependence<'a, M: ModuleStorage> {
    pub module: &'a mut M,
    pub stateless: StatelessBatch<'a, M>,
    pub stateful: StatefulBatch<'a, M>,
}

pub struct RContext<'a, M: ModuleStorage> {
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a Stateful<M>,
    // pub deps: RDependence<'a, M>,
}

pub struct AContext<'a, M: ModuleStorage + Dependence + 'a> {
    pub events: EventContext<'a>,
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a mut Stateful<M>,
    pub deps: AppDependence<'a, M>,
}

pub struct TContext<'a, M: ModuleStorage>
where
    Self: 'a,
{
    pub events: EventContext<'a>,
    pub stateless: StatelessBatch<'a, M>,
    pub stateful: StatefulBatch<'a, M>,
    // pub deps: TDependence<'a, M>,
}
