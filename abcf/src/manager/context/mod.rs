use crate::{entry::EventContext, Stateful, StatefulBatch, Stateless, StatelessBatch};

use super::ModuleStorage;

pub trait Dependence<'a> {
    type RPC;

    type App;

    type Txn;
}

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

pub struct AContext<'a, M: ModuleStorage> {
    pub events: EventContext<'a>,
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a mut Stateful<M>,
    // pub deps: ADependence<'a, M>,
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
