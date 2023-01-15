use super::StatusCode;
use std::{fmt::{Display, Formatter, Result as FmtResult}, net::TcpStream};
use std::io::{Write, Result as IoResult};

pub struct Response {
    status_code: StatusCode,
    body: Option<String>,
}

impl Response {
    pub fn new (status_code: StatusCode, body: Option<String>) -> Self {
        Response { status_code, body }
    }
    //dyn keyword-> Dynamic dispatch (resolved at runtime)
    //impl keyword-> Static dispatchh (resolved at compile time), will slow down compilation
    pub fn send (&self, stream: &mut impl Write) -> IoResult<()>{
        let body = match &self.body {
            Some(b) => b,
            None => ""
        };
        write!(stream, "HTTP/1.1 {} {} \r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
    }
}
