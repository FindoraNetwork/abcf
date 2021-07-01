// use abcf_core::message;
use crate::convert::Convert;
use abcf_core::{Application, Transaction};
use async_abci::abci;

use std::marker::PhantomData;

pub struct ApplicationWrapper<A: Application<T>, T: Transaction> {
    app: A,
    _marker: PhantomData<T>,
}

impl<A, T> ApplicationWrapper<A, T>
where
    A: Application<T>,
    T: Transaction,
{
    pub fn new(app: A) -> Self {
        Self {
            app,
            _marker: PhantomData,
        }
    }
}

#[async_trait::async_trait]
impl<A, T> abci::Application for ApplicationWrapper<A, T>
where
    A: Application<T>,
    T: Transaction,
{
    async fn echo(&mut self, request: abci::RequestEcho) -> abci::ResponseEcho {
        Application::echo(&mut self.app, request.convert())
            .await
            .convert()
    }

    async fn info(&mut self, request: abci::RequestInfo) -> abci::ResponseInfo {
        Application::info(&mut self.app, request.convert())
            .await
            .convert()
    }
}
