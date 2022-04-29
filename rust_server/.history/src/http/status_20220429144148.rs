use std::fmt::Display;

pub enum HttpStatus {
          OK = 200,
          BadRequest = 400,
          NotFound = 404,
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
                  Self::OK => "200 OK",
                  Self::BadRequest => "400 Bad Request",
                  self::NotFound => "404 Not Found",
        };
    }
}