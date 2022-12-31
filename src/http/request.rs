use super::{Method, MethodError,QueryString};
use std::convert::TryFrom;
use std::str;
use std::str::Utf8Error;
use std::fmt::{Debug, Display, Result as FmtResult, Formatter};

#[derive(Debug)]
pub struct Request<'buffer_lifetime> {
    method: Method,
    path: &'buffer_lifetime str,
    query_string: Option<QueryString<'buffer_lifetime>>
}

impl<'buffer_lifetime> Request<'buffer_lifetime> {
    pub fn method (&'buffer_lifetime self) -> &Method {
        &self.method
    }

    pub fn query_string (&self) -> Option<&QueryString> {
        self.query_string.as_ref()
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

impl<'buffer_lifetime> TryFrom<&'buffer_lifetime [u8]> for Request<'buffer_lifetime> {
    type Error = ParseError;

    fn try_from(buffer: &'buffer_lifetime [u8]) -> Result<Self, Self::Error> {

        let request = str::from_utf8(buffer)?;

        let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        
        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol)
        }

        let method: Method = method.parse()?;

        let mut query_string = None;

        if let Some(i) = path.find("?") {
            query_string = Some(QueryString::from(&path[i+1..]));
            path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method
        })
    }
}


fn get_next_word(request: &str) -> Option<(&str, &str)> {
    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i+1..]))
        }
    }
    None
}

pub enum ParseError {
    InvalidRequest,
    InvalidProtocol,
    InvalidMethod,
    InvalidEncoding
}

impl ParseError {
    fn message(&self) -> &str {
        match self {
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidRequest => "Invalid Request"
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
