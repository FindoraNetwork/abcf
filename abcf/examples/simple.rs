use abcf::Event;

/// Module's Event
#[derive(Debug, Event)]
pub struct Event1 {}

pub struct MockModule {}

/// Module's rpc.
#[abcf::rpcs]
impl MockModule {}

/// Module's methods.
impl MockModule {}

fn main() {}
