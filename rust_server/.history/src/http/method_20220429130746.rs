use stf::str::FromStr;
use crate::http::Error;
use crate::http::Result;

pub enum Method {
          GET,
          POST,
          PUT,
          DELETE,
}

impl FromStr for Method {
          type Err = Error;
          fn from_str(s: &str) -> Result<Self> {
                    match s {
                              "GET" =>Ok(Method::GET),
                              "POST" =>Ok(Method::POST),
                              "PUT" =>Ok(Method::PUT),
                              "DELETE" =>Ok(Method::DELETE),
                              _
                    }
          }
}
