/// Module.
pub trait Module {
    fn metadata(&self) -> ModuleMetadata<'_>;
}

/// Metadata of module.
pub struct ModuleMetadata<'a> {
    /// Name of module.
    pub name: &'a str,
    /// Version of module. If this version change, means module need update.
    pub version: &'a str,
    /// Version of impl. If this version change, means module only a change of impl.
    pub impl_version: &'a str,
    /// Genesis info.
    pub genesis: Genesis,
}

/// Genesis for module.
pub struct Genesis {
    pub target_hight: u64,
}
