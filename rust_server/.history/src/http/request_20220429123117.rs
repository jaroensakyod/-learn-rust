use crate::http::Method;
use std::convert::TryFrom;
use crate::http::Q
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}

impl TryFrom<&[u8]> for Request {

}