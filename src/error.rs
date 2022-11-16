use std::io;
use std::io::Error;
use failure::Fail;

#[derive(Debug, Fail)]
pub enum KvsError {

    #[fail(display= "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::Error),

    #[fail(display = "Key not found")]
    KeyNotFound,

    #[fail(display = "Unexpect command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: Error) -> Self {
        KvsError::Io(err)
    }
}

impl From<serde_json::Error> for KvsError {
    fn from(err: serde_json::Error) -> Self {
        KvsError::Serde(err)
    }
}

pub type Result<T> = std::result::Result<T, KvsError>;