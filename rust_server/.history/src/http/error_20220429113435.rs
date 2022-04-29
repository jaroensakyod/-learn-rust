use str::convert::From;

pub enum Error {
          InvalidRequest,
          InvalidProtocol,
          InvalidMethod,
          IO(String),
          Utf8(String),
}

impl From<std::io::Error> for Error {
          fn from(_: T) -> Self { 
                    todo!() 
          }
}