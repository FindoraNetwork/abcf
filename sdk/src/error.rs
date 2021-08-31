/// Error of abcf.
#[derive(Debug)]
pub enum Error {
    FromBytesError,
    JsonError(serde_json::Error),
    ReqWest(reqwest::Error),
    AbcfError(abcf::Error),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqWest(e)
    }
}

impl From<abcf::Error> for Error {
    fn from(e: abcf::Error) -> Self {
        Error::AbcfError(e)
    }
}

impl Error {
    pub fn to_code(&self) -> u32 {
        match self {
            Error::FromBytesError => 10001,
            Error::JsonError(_) => 10002,
            Error::ReqWest(_) => 10003,
            Error::AbcfError(_) => 10004,
        }
    }
}

/// Re-export `Result` for abcf.
pub type Result<T> = core::result::Result<T, Error>;
