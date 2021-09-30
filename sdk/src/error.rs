use alloc::string::ToString;

/// Error of abcf.
#[derive(Debug)]
pub enum Error {
    FromBytesError,
    JsonError(serde_json::Error),
    AbcfError(abcf::Error),
    ErrorString(alloc::string::String),
    RPCError(serde_json::Value),
    ReturnError(crate::jsonrpc::endpoint::Response),
    NotImpl,

    #[cfg(feature = "http")]
    ReqWest(reqwest::Error),

    #[cfg(feature = "websocket")]
    WebsocketError(async_tungstenite::tungstenite::Error),

    #[cfg(feature = "websocket")]
    FromUtf8Error(alloc::string::FromUtf8Error),
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::JsonError(e)
    }
}

#[cfg(feature = "http")]
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::ReqWest(e)
    }
}

impl From<serde_json::Value> for Error {
    fn from(e: serde_json::Value) -> Self {
        Error::RPCError(e)
    }
}

impl From<abcf::Error> for Error {
    fn from(e: abcf::Error) -> Self {
        Error::AbcfError(e)
    }
}

#[cfg(feature = "websocket")]
impl From<async_tungstenite::tungstenite::Error> for Error {
    fn from(e: async_tungstenite::tungstenite::error::Error) -> Self {
        Error::WebsocketError(e)
    }
}

impl From<&str> for Error {
    fn from(e: &str) -> Self {
        Error::ErrorString(e.to_string())
    }
}

#[cfg(feature = "websocket")]
impl From<alloc::string::FromUtf8Error> for Error {
    fn from(e: alloc::string::FromUtf8Error) -> Self {
        Error::FromUtf8Error(e)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
