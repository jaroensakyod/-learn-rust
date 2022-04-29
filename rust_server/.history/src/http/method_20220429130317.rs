use stf::str::FromStr;
use crate::http::Error;


pub enum Method {
          GET,
          POST,
          PUT,
          DELETE,
}