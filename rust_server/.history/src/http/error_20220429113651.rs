use str::convert::From;

pub enum Error {
          InvalidRequest,
          InvalidProtocol,
          InvalidMethod,
          IO(String),
          Utf8(String),
}

impl From<std::io::Error> for Error {
          fn from(e: std::io::Error) -> Self {
                    Self::IO(e.to_string())
          }
}

impl From<std::str::UtfError> for Error