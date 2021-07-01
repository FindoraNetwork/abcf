use crate::message::{check_tx, echo, info};
use crate::Transaction;

use alloc::boxed::Box;

#[async_trait::async_trait]
pub trait Application<T: Transaction>: Send {
    async fn echo(&mut self, _req: echo::Request) -> echo::Response {
        echo::Response::default()
    }

    async fn info(&mut self, _req: info::Request) -> info::Response {
        info::Response::default()
    }

    // ...

    async fn check_tx(&mut self, _req: check_tx::Request<T>) -> check_tx::Response
    where
        T: 'async_trait,
    {
        check_tx::Response::default()
    }

    // ...
}

impl<T: Transaction> Application<T> for () {}
