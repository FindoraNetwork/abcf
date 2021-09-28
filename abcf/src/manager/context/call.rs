use core::any::Any;

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

pub struct CallEntry {
    pub method: String,
    pub args: Box<dyn Any + Sync + Send>,
}

pub struct CallContext<'a> {
    pub entries: &'a mut BTreeMap<String, Vec<CallEntry>>,
}

impl<'a> CallContext<'a> {
    pub fn new(i: &'a mut CallImpl) -> Self {
        Self {
            entries: &mut i.entries,
        }
    }

    pub fn pop_module_calls(&mut self, name: &str) -> Option<Vec<CallEntry>> {
        self.entries.remove(name)
    }

    pub fn push_module_call(&mut self, name: &str, call: CallEntry) {
        if let Some(v) = self.entries.get_mut(name) {
            v.push(call);
        } else {
            let mut value = Vec::new();
            value.push(call);
            self.entries.insert(String::from(name), value);
        }
    }
}

pub struct CallImpl {
    pub entries: BTreeMap<String, Vec<CallEntry>>,
}

impl CallImpl {
    pub fn new() -> Self {
        Self {
            entries: BTreeMap::new(),
        }
    }
}
