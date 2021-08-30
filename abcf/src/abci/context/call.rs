use core::any::Any;

use alloc::{boxed::Box, collections::BTreeMap, string::String, vec::Vec};

use crate::{module::Callable, Error, Result};

pub struct CallContext<'a> {
    pub name_index: &'a BTreeMap<String, usize>,
    pub calls: &'a mut Vec<Box<dyn Any + Send + Sync>>,
}

impl<'a> CallContext<'a> {
    pub fn get_module<C: Callable + 'static>(&mut self, name: &str) -> Result<&mut C> {
        let index = self.name_index.get(name).ok_or(Error::NoModule)?;
        let calls = self.calls.get_mut(*index).ok_or(Error::NoModule)?;
        let calls_mut = calls.as_mut() as &mut dyn Any;
        Ok(calls_mut.downcast_mut::<C>().ok_or(Error::NoModule)?)
    }
}
