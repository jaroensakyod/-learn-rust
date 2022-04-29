use std::convert::From;
use std::fmt::Display;

pub enum Error {
          InvalidRequest,
          InvalidProtocol,
          InvalidMethod,
          IO(String),
          Utf8(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
                  Self::InvalidRequest => "Invalid request",
                  Self::InvalidProtocol => "Invalid protocol",
                  Self::InvalidMethod => "Invalid method",
                  Se
        };
    }
}

impl From<std::io::Error> for Error {
          fn from(e: std::io::Error) -> Self {
                    Self::IO(e.to_string())
          }
}

impl From<std::str::Utf8Error> for Error {
          fn from(e: std::str::Utf8Error) -> Self {
                    Self::Utf8(e.to_string())
          }
}