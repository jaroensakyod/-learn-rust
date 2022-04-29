use std::net::TcpListener;
use std::io::Read;
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
                              let mut buf: [u8; 1024] = [0; ]


                    }

                    Ok(())

                    
          }
}