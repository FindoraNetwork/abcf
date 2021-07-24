use alloc::string::String;

use super::{Application, Events, RPCs, Storages};

pub trait Module {
    type Storages: Storages;

    type Events: Events;

    type RPCs: RPCs;

    // type Transaction: Transaction;
    type Application: Application;

    // fn name(&self) -> &str;
    //
    fn metadata(&self) -> ModuleMetadata;

    fn application(&self) -> Self::Application;
    fn events(&self) -> Self::Events;
    fn rpcs(&self) -> Self::RPCs;
    fn storages(&self) -> Self::Storages;
}

pub struct ModuleMetadata {
    pub name: String,
    pub version: String,
    pub impl_version: String,
}
