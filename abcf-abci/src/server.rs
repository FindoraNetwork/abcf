use abcf_core::{Application, Transaction};
use crate::application::ApplicationWrapper;
use tokio::net::{ToSocketAddrs};

pub struct Server<App: Application<T>, T: Transaction + Send> {
    wrapper: ApplicationWrapper<App, T>,
    server: Option<async_abci::Server<ApplicationWrapper<App, T>>>,
}

impl<App: Application<T>, T: Transaction + Send> Server<App, T> {
    pub fn new(app: App) -> Self {
        Self {
            wrapper: ApplicationWrapper::new(app),
            server: None
        }
    }

    pub async fn bind<Addr: ToSocketAddrs>(mut self, addr: Addr) -> Result<Self> {
        let server = async_abci::Server::bind(addr).await?;
        self.server = server;
        Ok(server)
    }
}

