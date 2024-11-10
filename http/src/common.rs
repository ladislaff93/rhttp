use core::{error, fmt};
use std::str::Utf8Error;

pub const FINAL_CRLF: &str = "\r\n\r\n";
pub const CRLF: &str = "\r\n";

#[derive(Debug)]
pub struct ParsingRequestErr(String);

impl fmt::Display for ParsingRequestErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while parsing request! {}", self.0)
    }
}

/*
* use thiserror crate to handle errors 
*
* */

impl error::Error for ParsingRequestErr {}

#[derive(Debug)]
pub enum RhttpErr {
    ParsingRequestErr,
    ParsingHttpMethodErr,
    ParsingPathErr,
    ParsingHttpProtocolErr,
    ParsingHttpHeaderErr,
    HeaderValueErr(Utf8Error),
    ParsingContentLength(core::convert::Infallible)
}

impl From<core::convert::Infallible> for RhttpErr {
    fn from(value: core::convert::Infallible) -> Self {
       Self::ParsingContentLength(value) 
    }
}

impl From<Utf8Error> for RhttpErr {
    fn from(value: Utf8Error) -> Self {
        Self::HeaderValueErr(value)
    }
}
