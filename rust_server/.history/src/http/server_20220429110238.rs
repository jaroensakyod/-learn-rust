use std::net::TcpListener;
use std::io::Read;
use std::str;
pub struct Server {
          addr: String,
}

impl Server {
          pub fn new(addr: String) ->Self {
                    Self { addr }
          }

          pub fn run(&self) -> std::io::Result<()> {
                    println!("Listering in {}",self.addr);

                    
                    let listener = TcpListener::bind(&self.addr)?;

                    for stream in listener.incoming(){
                              let mut stream = stream?;
                              let mut buf: [u8; 1024] = [0; 1024];
                              stream.read(&mut buf)?;
                              let request = str::from_utf8(&str)

                    }

                    Ok(())

                    
          }
}