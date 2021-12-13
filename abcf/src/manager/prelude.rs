use crate::{entry::Tree, module::StorageTransaction, Storage};

pub trait ModuleStorage {
    type Stateless: Storage + StorageTransaction + Tree;

    type Stateful: Storage + StorageTransaction + Tree;
}

// pub trait ModuleStorageDependence<'a> {
// type Dependence: Send;
// }
