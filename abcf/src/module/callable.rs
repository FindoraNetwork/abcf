use core::any::Any;

pub trait Callable: Send + Sync + Any {}

impl Callable for () {}
