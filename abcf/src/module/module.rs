use alloc::string::String;

use super::{Application, Event, RPCs, Storages};

pub trait Module {
    // type Storages: Storages;

    // type Events: Events;

    type RPCs: RPCs;

    type Application: Application;

    fn metadata(&self) -> ModuleMetadata;

    fn application(&self) -> Self::Application;
    // fn events(&self) -> Self::Events;
    fn rpcs(&self) -> Self::RPCs;
    // fn storages(&self) -> Self::Storages;
}

pub struct ModuleMetadata {
    pub name: String,
    pub version: String,
    pub impl_version: String,
}
