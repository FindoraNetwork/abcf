// use crate::ModuleApplications;
use abcf_core::message::{check_tx};
use abcf_core::{Application, Module, Transaction};

pub struct Manager<'a, T: Transaction> {
    // This allow beacuse rust compiler impl not complete.
    // ref: https://doc.rust-lang.org/reference/items/traits.html#object-safety
    #[allow(where_clauses_object_safety)]
    apps: Vec<&'a mut (dyn Application<T> + Sync + Send)>,
}

impl<'a, T: Transaction> Manager<'a, T> {
    pub fn new() -> Self {
        Manager { apps: Vec::new() }
    }

    pub fn regist<M, A>(&mut self, module: &'a mut M)
    where
        M: Module<Transaction = T, Application = A>,
        T: Transaction,
        A: Application<T> + 'a + Sync + Send,
    {
        self.apps.push(module.application());
    }

    pub async fn dispatch_check_tx(&mut self, req: check_tx::Request<T>) -> check_tx::Response
    where
        T: Sync,
    {
        for app in &mut self.apps {
            let res = app.check_tx(&req).await;
            if res.code != 0 {
                return res
            }
        }

        check_tx::Response::default()
    }
}
