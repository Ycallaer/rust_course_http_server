
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::Display;
use std::fmt::Debug;
use std::fmt::Result as FmtResult;
use std::fmt::Formatter;


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

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {    
        unimplemented!()
    }
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