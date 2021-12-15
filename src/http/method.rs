use std::str::FromStr;

#[derive(Debug)]
pub enum Method {
    CONNECT,
    OPTIONS,
    DELETE,
    PATCH,
    TRACE,
    HEAD,
    POST,
    GET,
    PUT,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONNECT" => Ok(Self::CONNECT),
            "OPTIONS" => Ok(Self::OPTIONS),
            "DELETE" => Ok(Self::DELETE),
            "PATCH" => Ok(Self::PATCH),
            "TRACE" => Ok(Self::TRACE),
            "HEAD" => Ok(Self::HEAD),
            "POST" => Ok(Self::POST),
            "GET" => Ok(Self::GET),
            "PUT" => Ok(Self::PUT),
            _ => Err(MethodError),
        }
    }
}

pub struct MethodError;