use crate::http::Method;
pub struct Request {
          method: Method,
          path: String,
          query_string: QueryString,
}