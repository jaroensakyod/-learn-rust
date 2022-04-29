use std::fmt::Display;

pub enum HttpStatus {
          OK = 200,
          BadRequest = 400,
          NotFound = 404,
}

impl Display for Http