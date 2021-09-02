use abcf::Event;

/// Module's Event
#[derive(Debug, Event)]
pub struct Event1 {}
#[abcf::module(
    name = "mock",
    version = "0.1",
    impl_version = "0.1.1",
    target_height = 0
)]
pub struct MockModule {}

/// Module's rpc.
#[abcf::rpcs]
impl MockModule {}

/// Module's methods.
impl MockModule {}

fn main() {}
