use crate::http::Method;
use std::convert::TryFrom;
use crate::http::QueryString;
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}

impl TryFrom<&[u8]> for Request {

type Error = Type;
fn try_from(_: T) -> std::result::Result<Self, <Self as std::convert::TryFrom<T>>::Error> { todo!() }
}