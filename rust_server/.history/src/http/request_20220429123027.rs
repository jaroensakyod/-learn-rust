use crate::http::Method;
use std::convert::TryFrom;
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}

impl TryFrom<&>