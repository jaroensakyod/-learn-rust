use crate::http::Method;
use std::convert::From
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}