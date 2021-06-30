use abcf_core::{Application, Transaction};

pub struct ModuleApplications<'a, T: Transaction> {
}

impl<'a, T: Transaction> ModuleApplications<'a, T> {
    pub fn new() -> Self {
        ModuleApplications { apps: Vec::new() }
    }

    pub fn push<A>(&mut self, app: &'a mut A)
    where
        A: Application<T>,
    {
        self.apps.push(app);
    }
}
