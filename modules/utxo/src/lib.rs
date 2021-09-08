
/// Module's Event
#[derive(Clone, Debug, Deserialize, Serialize, Event)]
pub struct Event1 {}

#[abcf::module(name = "mock", version = 1, impl_version = "0.1.1", target_height = 0)]
pub struct MockModule<S, D>
where
    S: abcf::bs3::Store,
    D: digest::Digest,
{
    // /// In memory.
    pub inner: u32,
    pub maker_s: PhantomData<S>,
    pub maker_d: PhantomData<D>,
    #[stateful]
    pub sf_value: BValue<u32>,
    #[stateless]
    pub sl_value: BValue<u32>,
    #[stateless]
    pub sl_map: Map<i32, u32>,
}

#[abcf::rpcs(module = "mock")]
impl<S, D> MockModule<S, D> 
where
    S: abcf::bs3::Store,
    D: digest::Digest,
{}


/// Module's block logic.
#[abcf::application]
impl<S, D> Application<abcf::Stateless<Self>, abcf::Stateful<Self>> for MockModule<S, D>
where
    S: abcf::bs3::Store + 'static,
    D: digest::Digest + Send + Sync,
{
}

/// Module's methods.
impl<S, D> MockModule<S, D>
where
    S: abcf::bs3::Store,
    D: digest::Digest,
{
}
