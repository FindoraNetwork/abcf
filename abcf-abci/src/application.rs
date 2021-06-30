use abcf_core::message;
use abcf_core::{Application, Transaction};
use async_abci::abci;

use std::marker::PhantomData;

pub struct ApplicationWrapper<A: Application<T>, T: Transaction> {
    app: A,
    _marker: PhantomData<T>,
}

impl<A, T> ApplicationWrapper<A, T> where A: Application<T>, T: Transaction {
    pub fn new(app: A) -> Self {
        Self {
            app,
            _marker: PhantomData
        }
    }
}

#[async_trait::async_trait]
impl<A, T> abci::Application for ApplicationWrapper<A, T>
where
    A: Application<T> + Send + 'static,
    T: Transaction + Send + 'static,
{
    async fn echo(&mut self, request: abci::RequestEcho) -> abci::ResponseEcho {
        let resp = Application::echo(
            &mut self.app,
            message::echo::Request {
                message: request.message,
            },
        )
        .await;
        abci::ResponseEcho {
            message: resp.message,
        }
    }
}
