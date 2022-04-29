use std::collections::HashMap;
use std::convert::From;

pub struct QueryString{
          data: HashMap<String, String>,
}

impl QueryString {
          pub fn get(&self, key:&S)
}

impl Form<&str> for QueryString {
          fn from(s: &str) -> Self{
                    let mut data = HashMap::new();

                    //give name=bond & age=18
                    for c in s.split('&') {
                              if let Some(i) = c.find('=') {
                                        let key = &c[..i];
                                        let value = &c[i + 1..];
                                        data.insert(key.to_owned(), value.to_owned());
                              }
                    }
                    QueryString{ data }
          }
}