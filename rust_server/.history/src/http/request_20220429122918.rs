use crate::http::Method;
use std::
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}