pub(crate) mod application;

mod server;
pub use server::Server;

pub(crate) mod convert;

#[derive(Debug)]
pub enum Error {
    AsyncAbciError(async_abci::Error),
}

impl From<async_abci::Error> for Error {
    fn from(e: async_abci::Error) -> Self {
        Error::AsyncAbciError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
