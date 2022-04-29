use stf::str::FromStr;
use crate::http::Error;
use crate::http::Result;

pub enum Method {
          GET,
          POST,
          PUT,
          DELETE,
}

impl From for Method {
          type Err = Error;
          fn
}