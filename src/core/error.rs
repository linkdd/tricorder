use std::{
  fmt::{Display, Formatter, Result},
  error::Error as BaseError,
};

#[derive(Debug, Clone)]
pub enum Error {
  MissingInput(String),
  CommandExecutionFailed(String),
  FileNotFound(String),
  IsADirectory(String),
  IsAbsolute(String),
  InvalidHostId(String),
  InvalidHostTag(String),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{:?}", self)
  }
}

impl BaseError for Error {}