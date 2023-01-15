use std::{net::TcpListener, io::Read, io::Write};
use crate::http::{Request, Response, StatusCode}; //crate keyword goes to the root
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
                            let response = match Request::try_from(&buffer[..] ) {
                                Ok(Request) => {
                                    Response::new(StatusCode::Ok, Some("<h1>It works</h1>".to_string()))
                                }
                                Err(e) => {
                                    println!("Failed to parse a request {}",e);
                                    Response::new(StatusCode::BadRequest, None)
                                }
                            };
                            if let Err(e) = response.send(&mut stream){
                                println!("Failed to send reponse {}",e);
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