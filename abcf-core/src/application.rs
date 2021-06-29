use crate::message::{check_tx, info};
use crate::Transaction;

#[async_trait::async_trait]
pub trait Application<T: Transaction> {
    async fn info(&mut self, _req: &info::Request) -> info::Response {
        info::Response::default()
    }

    // ...

    async fn check_tx(&mut self, _req: &check_tx::Request<T>) -> check_tx::Response {
        check_tx::Response::default()
    }

    // ...
}
