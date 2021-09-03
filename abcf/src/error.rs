use alloc::{format, string::String};

/// Error of abcf.
#[derive(Debug)]
pub enum Error {
    FromBytesError,
    JsonError(serde_json::Error),
    QueryPathFormatError,
    NoModule,
    NoRPCMethod,

    RPCApplicationError(u32, String),
    ABCIApplicationError(u32, String),
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

    pub fn code(&self) -> u32 {
        match self {
            Error::FromBytesError => 10001,
            Error::JsonError(_) => 10002,
            Error::QueryPathFormatError => 10004,
            Error::NoModule => 10005,
            Error::NoRPCMethod => 10006,

            Error::TempOnlySupportRPC => 90001,
            Error::RPCApplicationError(code, _) => code.clone(),
            Error::ABCIApplicationError(code, _) => code.clone(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            Self::FromBytesError => String::from(""),
            Self::JsonError(e) => format!("{:?}", e),
            Self::QueryPathFormatError => String::from("query path error"),
            Self::NoModule => String::from("no module"),
            Self::NoRPCMethod => String::from("no rpc method"),
            Self::RPCApplicationError(_, m) => m.clone(),
            Self::ABCIApplicationError(_, m) => m.clone(),
            Self::TempOnlySupportRPC => String::from(""),
        }
    }
}

/// Re-export `Result` for abcf.
pub type Result<T> = core::result::Result<T, Error>;

pub struct ModuleError {
    pub namespace: String,
    pub error: Error,
}

impl ModuleError {
    pub fn new(namespace: &str, e: Error) -> Self {
        Self {
            namespace: String::from(namespace),
            error: e,
        }
    }
}

pub type ModuleResult<T> = core::result::Result<T, ModuleError>;
