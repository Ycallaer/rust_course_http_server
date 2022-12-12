
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Result as FmtResult;
use std::fmt::Formatter;
use std::str;
use std::str::Utf8Error;


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

// impl Request {
//     fn from_byte_array(buf: &[u8]) -> Result<Self, String> {}
// }

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;

    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        //the question mark is shorthand for the match operator    
        let request = str::from_utf8(buf)?;
        unimplemented!()
    }
}

fn get_next_word(request: &str) -> Option<(&str,&str)>{
    for (i,c) in request.chars().enumerate(){
        if c ==' '{
            return Some((&request[..i],&request[i+1..]))
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidRequest => "Invalid Request",
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidMethod => "Invalid Method",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Error for ParseError {

}

impl Display for ParseError  {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())   
    }
}

impl Debug for ParseError  {
    fn fmt(&self, f: &mut Formatter)  -> FmtResult {
        write!(f, "{}", self.message())   
    }
}