use crate::http::Method;
use std::con
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}