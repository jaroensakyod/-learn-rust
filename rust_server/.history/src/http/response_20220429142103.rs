use crate::http::HttpStatus;
pub struct Response {
          http_status: HttpStatus,
          body: Option<String>,
}

impl Response {
          pub fn new(http_status: HttpStatus, body: Option<String>) -> Self {
                    Self { http_status, body }
          }
}