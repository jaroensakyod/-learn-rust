use crate::http::Method;
use std::convert::TryFrom;
use crate::http::QueryString;
use crate::http::Error;
use crate::http::Result;
use std::str;
pub struct Request {
          method: Method,
          path: String,
          query_string: Option<QueryString>,
}

impl TryFrom<&[u8]> for Request {

          type Error = Error;
          fn try_from(buf: &[u8]) -> Result<Self> {
                    
                    let request = str::from_utf8(&buf)?;
                    println!("{}", request);

                    //GET /hello?name=bond&age=18 HTTP/1.1
                    let mut request = request.split_whitespace();
                    let method = request.next().ok_or(Error::InvalidRequest)?;
                    let path = request.next().ok_or(Error::InvalidRequest)?;
                    let protocol = request.next().ok_or(Error::InvalidRequest)?;

                    if protocol != "HTTP/1.1" {
                              return Err(Error::InvalidProtocol);
                    }
                    //&str->enum 
                    let method: Method = method.parse()?;
                    //query_string

                    match path.find('?'){
                              Some(i) => {}
                    }

                              
                    }

                    Ok(Self{
                              method: Method::GET,
                              path: "/".to_string(),
                              query_string: None,
                    })
          }
}