use std::{
    error::Error as BaseError,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug, Clone)]
pub enum Error {
    MissingInput(String),
    CommandExecutionFailed(String),
    UploadFailed(String),
    FileNotFound(String),
    IsADirectory(String),
    IsAbsolute(String),
    InvalidHostId(String),
    InvalidHostTag(String),
    InvalidToken(String),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}

impl BaseError for Error {}
