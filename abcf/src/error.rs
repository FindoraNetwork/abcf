use alloc::string::String;

/// Error of abcf.
#[derive(Debug)]
pub enum Error {
    FromBytesError,
    JsonParseError,
    JsonDumpError,
    QueryPathFormatError,
    RPRApplicationError(u32, String),
    TempOnlySupportRPC,
}

impl Error {
    pub fn to_code(&self) -> u32 {
        match self {
            Error::FromBytesError => 10001,
            Error::JsonParseError => 10002,
            Error::JsonDumpError => 10003,
            Error::QueryPathFormatError => 10004,
            Error::TempOnlySupportRPC => 90001,
            Error::RPRApplicationError(code, _) => code.clone(),
        }
    }
}

/// Re-export `Result` for abcf.
pub type Result<T> = core::result::Result<T, Error>;
