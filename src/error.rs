use std;
use reqwest;

#[derive(Debug)]
pub enum Error {
  Io(std::io::Error),
  Reqwest(reqwest::Error),
}

impl From<std::io::Error> for Error {
  fn from(error: std::io::Error) -> Self {
    Error::Io(error)
  }
}

impl From<reqwest::Error> for Error {
  fn from(error: reqwest::Error) -> Self {
    Error::Reqwest(error)
  }
}
