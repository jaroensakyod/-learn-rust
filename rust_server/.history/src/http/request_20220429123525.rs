use crate::http::Method;
use std::convert::TryFrom;
use crate::http::QueryString;
use crate::http::Error;
use crate::http::Result;
use s
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}

impl TryFrom<&[u8]> for Request {

          type Error = Error;
          fn try_from(buf: &[u8]) -> Result<Self> {
                    
                    let request = try!()
                    Ol(Self{
                              method: Method::GET,
                              path: "/".to_string(),
                              query_string: None,
                    })
          }
}