use std::collections::HashMap;
use std::convert::From;

pub struct QueryString{
          data: HashMap<String, String>,
}

impl Form<&str> for QueryString {
          fn from(s: &str) -> Self{
                    let mut data = HashMap::new();

                    //give name=bond & age=18
                    for c in s.split('&') {
                              c.find('=')
                    }
          }
}