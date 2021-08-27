#[derive(Debug)]
pub enum Error {
    TendermintError(tendermint_sys::Error),
    PathError,
}

impl From<tendermint_sys::Error> for Error {
    fn from(e: tendermint_sys::Error) -> Self {
        Error::TendermintError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
