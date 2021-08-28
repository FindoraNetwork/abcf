use alloc::string::String;

/// Error of abcf.
#[derive(Debug)]
pub enum Error {
    FromBytesError,
    JsonError(serde_json::Error),
    QueryPathFormatError,
    NoModule,
    NoRPCMethod,

    RPCApplicationError(u32, String),
    TempOnlySupportRPC,
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e)
    }
}

impl Error {
    pub fn new_rpc_error(code: u32, message: &str) -> Self {
        Self::RPCApplicationError(code, String::from(message))
    }

    pub fn to_code(&self) -> u32 {
        match self {
            Error::FromBytesError => 10001,
            Error::JsonError(_) => 10002,
            Error::QueryPathFormatError => 10004,
            Error::NoModule => 10005,
            Error::NoRPCMethod => 10006,

            Error::TempOnlySupportRPC => 90001,
            Error::RPCApplicationError(code, _) => code.clone(),
        }
    }
}

/// Re-export `Result` for abcf.
pub type Result<T> = core::result::Result<T, Error>;
