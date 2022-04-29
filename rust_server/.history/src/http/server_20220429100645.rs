use std::net

pub struct Server {
          addr: String,
}

impl Server {
          pub fn new(addr: String) ->Self {
                    Self { addr }
          }

          pub fn run(&self) {
                    println!("Listering in {}",self.addr);
          }
}