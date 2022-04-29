use crate::http::HttpStatus;
use std::net::TcpStream;
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
                              Some(v) => v,
                              None => "",
                    };
                    write!(stream, "HTTP/1.1 404 {}\r\n\r\n{}",self.http_status, body)
          }
}