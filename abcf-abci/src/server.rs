use crate::application::ApplicationWrapper;
use crate::Result;
use abcf_core::{Application, Transaction};
use tokio::net::ToSocketAddrs;

pub struct Server<App: Application<T>, T: Transaction> {
    wrapper: ApplicationWrapper<App, T>,
}

impl<App: Application<T> + 'static, T: Transaction + 'static> Server<App, T> {
    pub fn new(app: App) -> Self {
        Self {
            wrapper: ApplicationWrapper::new(app),
        }
    }

    pub async fn bind<Addr: ToSocketAddrs>(
        self,
        addr: Addr,
    ) -> Result<async_abci::Server<ApplicationWrapper<App, T>>> {
        let server = async_abci::Server::new(self.wrapper).bind(addr).await?;
        Ok(server)
    }
}
