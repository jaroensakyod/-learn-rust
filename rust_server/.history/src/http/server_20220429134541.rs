use std::net::TcpListener;
use std::io::Read;
use crate::http::Result;
use crate::http::Request;
pub struct Server {
          addr: String,
}

impl Server {
          pub fn new(addr: String) ->Self {
                    Self { addr }
          }

          pub fn run(&self) -> Result<()> {
                    println!("Listering in {}",self.addr);

                    
                    let listener = TcpListener::bind(&self.addr)?;

                    for stream in listener.incoming(){
                              let mut stream = stream?;
                              let mut buf: [u8; 1024] = [0; 1024];
                              stream.read(&mut buf)?;
                              
                              Request::try

                    }

                    Ok(())

                    
          }
}