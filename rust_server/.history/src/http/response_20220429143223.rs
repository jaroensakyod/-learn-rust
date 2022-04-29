use crate::http::HttpStatus;
use std::net::TcpStream;
use crate::http::Res
pub struct Response {
          http_status: HttpStatus,
          body: Option<String>,
}

impl Response {
          pub fn new(http_status: HttpStatus, body: Option<String>) -> Self {
                    Self { http_status, body }
          }

          pub fn send(&self, stream: &mut TcpStream) ->
}