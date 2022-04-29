use crate::http::HttpStatus;
pub struct Response {
          http_status: HttpStatus,
          body: Option<String>,
}

impl Response {
          pub fn new(status: HttpStatus, body: Option<String>) -> Self {
                    
          }
}