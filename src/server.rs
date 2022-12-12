use std::{net::TcpListener, io::Read};
use crate::http::Request; //crate keyword goes to the root
use std::convert::TryFrom;
use std::convert::TryInto;
pub struct Server{
    addr: String,
}

impl Server{
    //each struct has the keyword Self referencing itself
    pub fn new(addr: String)-> Self{
        Self { addr } // same as addr: addr
    }

    //methods use the first special param self
    pub fn run(self){
        println!("The server is running on {}",self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();
        
        //this is an infinite loop with as label outer
        'outer: loop {
            match listener.accept() {
                Ok((mut stream,_)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(& mut buffer){
                        Ok(_) => {
                            println!("Received from request: {} ", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..] ) {
                                OK(Request) => {}
                                Err(e) => println!("Failed to parse a request {}",e)
                            }
                        }
                        Err(e) => println!("Failed to read from connection. Error {}",e)
                    }
                }
                Err(e) => println!("Failed to establish a connection, error: {}",e)
            }
        }
    }
}