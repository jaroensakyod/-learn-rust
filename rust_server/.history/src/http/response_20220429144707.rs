use crate::http::HttpStatus;
use std::net::TcpStream;
use crate::http::Result;
use std::io::Write;
pub struct Response {
          http_status: HttpStatus,
          body: Option<String>,
}

impl Response {
          pub fn new(http_status: HttpStatus, body: Option<String>) -> Self {
                    Self { http_status, body }
          }

          pub fn send(&self, stream: &mut TcpStream) -> std::io::Result<()> {
                    let body = match &self.body {
                              Some(v) => v.get_unchecked(i)
                    }
                    write!(stream, "HTTP/1.1 404 Not Found\r\n\r\n")
          }
}