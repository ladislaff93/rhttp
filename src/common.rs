use core::{error, fmt};

pub const FINAL_CRLF: &str = "\r\n\r\n";
pub const CRLF: &str = "\r\n";

#[derive(Debug)]
pub struct ParsingRequestErr(String);

impl fmt::Display for ParsingRequestErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while parsing request! {}", self.0)
    }
}

impl error::Error for ParsingRequestErr {}

#[derive(Debug)]
pub enum RhttpErr {
    ParsingRequestErr,
    ParsingHttpMethodErr,
    ParsingPathErr,
    ParsingHttpProtocolErr,
    ParsingHttpHeaderErr,
    ParsingContentLength(core::convert::Infallible)
}

impl From<core::convert::Infallible> for RhttpErr {
    fn from(value: core::convert::Infallible) -> Self {
       Self::ParsingContentLength(value) 
    }
}
