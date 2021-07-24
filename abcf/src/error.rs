#[derive(Debug)]
pub enum Error {
    FromBytesError,
}

pub type Result<T> = core::result::Result<T, Error>;
