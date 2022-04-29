use crate::http::Method;
use std::convert::TryFrom;
use crate::http::QueryString;
use crate::http::Error;
use crate::http::Result;
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}

impl TryFrom<&[u8]> for Request {

          type Error = Error;
          fn try_from(buf: &[u8]) -> Result<Self> {
                    
                    Ol(Self{
                              method: Method::GET,
                              path: "/".to_string(),
                              query_string: M
                    })
          }
}