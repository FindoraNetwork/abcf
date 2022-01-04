pub mod context;
pub use context::{
    AContext, AppDependence, Dependence, RContext, RPCDependence, TContext, TxnDependence,
};

mod prelude;
pub use prelude::ModuleStorage;

/// Module.
pub trait Module {
    fn metadata(&self) -> ModuleMetadata<'_>;
}

pub enum ModuleType {
    Module,
    Manager,
}

/// Metadata of module.
pub struct ModuleMetadata<'a> {
    /// Name of module.
    pub name: &'a str,
    /// type of module.
    pub module_type: ModuleType,
    /// Version of module. If this version change, means module need update.
    pub version: u64,
    /// Version of impl. If this version change, means module only a change of impl.
    pub impl_version: &'a str,
    /// Genesis info.
    pub genesis: Genesis,
}

/// Genesis for module.
pub struct Genesis {
    pub target_height: u64,
}
