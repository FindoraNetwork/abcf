use core::any::Any;

use alloc::{boxed::Box, string::String, vec::Vec};

pub struct CallEntry {
    pub module: String,
    pub method: String,
    pub args: Vec<Box<dyn Any + Sync + Send>>,
}

pub struct CallContext<'a> {
    pub entries: &'a mut Vec<CallEntry>,
}

impl<'a> CallContext<'a> {
    pub fn new(i: &'a mut CallImpl) -> Self {
        Self {
            entries: &mut i.entries,
        }
    }
}

pub struct CallImpl {
    pub entries: Vec<CallEntry>,
}

impl CallImpl {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
}

