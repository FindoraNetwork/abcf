use crate::{entry::EventContext, Stateful, StatefulBatch, Stateless, StatelessBatch};

use super::ModuleStorage;

pub struct RDependence<'a, M, Sl, Sf> {
    pub module: &'a M,
    pub stateless: &'a mut Sl,
    pub stateful: &'a Sf,
}

pub struct Dependence<'a, M, Sl, Sf> {
    pub module: &'a mut M,
    pub stateless: &'a mut Sl,
    pub stateful: &'a mut Sf,
}

pub struct RContext<'a, M: ModuleStorage> {
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a Stateful<M>,
    pub deps: RDependence<'a, M, Stateless<M>, Stateful<M>>,
}

pub struct AContext<'a, M: ModuleStorage> {
    pub events: EventContext<'a>,
    pub stateless: &'a mut Stateless<M>,
    pub stateful: &'a mut Stateful<M>,
    pub deps: Dependence<'a, M, Stateless<M>, Stateful<M>>,
}

pub struct TContext<'a, M: ModuleStorage> {
    pub events: EventContext<'a>,
    pub stateless: StatelessBatch<'a, M>,
    pub stateful: StatefulBatch<'a, M>,
    pub deps: Dependence<'a, M, StatelessBatch<'a, M>, StatefulBatch<'a, M>>,
}
