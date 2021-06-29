mod codec;
pub use codec::ServerCodec;

use std::io;

pub enum Error {
    StdIoError(io::Error),
    ProstEncodeError(prost::EncodeError),
    ProstDecodeError(prost::DecodeError),
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::StdIoError(e)
    }
}

impl From<prost::EncodeError> for Error {
    fn from(e: prost::EncodeError) -> Self {
        Error::ProstEncodeError(e)
    }
}

impl From<prost::DecodeError> for Error {
    fn from(e: prost::DecodeError) -> Self {
        Error::ProstDecodeError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

