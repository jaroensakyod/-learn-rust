use str::convert::Fr

pub enum Error {
          InvalidRequest,
          InvalidProtocol,
          InvalidMethod,
          IO(String),
          Utf8(String),
}