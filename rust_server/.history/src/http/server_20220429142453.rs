use std::net::TcpListener;
use std::io::Read;
use crate::http::Result;
use crate::http::Request;
use std::fmt::Debug;
use crate::http::method::Method;
use crate::http::Response;
use crate::http::HttpStatus;

#[derive(Debug)]
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
                              
                              let request = Request::try_from(&buf[..])?;
                              println!("{:#?}",request);

                              let response = match request.method() {
                                        Method::GET => match request.path().as_str(){
                                                  "/" =>Response::new(HttpStatus::OK,Some("home").to_s)
                                                  "/hello" =>{}
                                                  _ =>Response::new(HttpStatus::NotFound,None>),
                                        }
                                        _ => Response::new(HttpStatus::NotFound,None>),
                              };

                    }

                    Ok(())

                    
          }
}