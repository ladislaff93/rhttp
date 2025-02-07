use std::str::Utf8Error;
use thiserror::Error;

pub const FINAL_CRLF: &str = "\r\n\r\n";
pub const CRLF: &str = "\r\n";

// #[derive(Debug)]
// pub struct ParsingRequestErr(String);
//
// impl fmt::Display for ParsingRequestErr {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "error while parsing request! {}", self.0)
//     }
// }

// impl error::Error for ParsingRequestErr {}

#[derive(Error, Debug)]
pub enum RhttpError {
    #[error("error while parsing request! {0}")]
    ParsingRequestErr(String),
    #[error("error while parsing http method! {}", self)]
    ParsingHttpMethodErr,
    #[error("error while parsing http path! {}", self)]
    ParsingPathErr,
    #[error("error while parsing http protocol! {}", self)]
    ParsingHttpProtocolErr,
    #[error("error while parsing http header! {}", self)]
    ParsingHttpHeaderErr,
    #[error("error while parsing http header value! {}", self)]
    HeaderValueErr(#[from] Utf8Error),
    #[error("listener already defined {}", self)]
    ListenerDefined,
    #[error("no listener defined {}", self)]
    ListenerNotDefined,
    #[error("no handler found for path {}", self)]
    HandlerNotFound(String),
    #[error("error while parsing request! {}", self)]
    ParsingContentLength(#[from] core::convert::Infallible),
    #[error("error while parsing request! {}", self)]
    UnableToBindAddress(#[from] std::io::Error),
    #[error("error while parsing query params! {}", self)]
    ParsingQueryParamsErr(#[from] serde_qs::Error),
    #[error("error while parsing path params! {}", self)]
    ParsingPathParamsErr,
    #[error("error while parsing widlcard params! {}", self)]
    WildCardPathParamsErr,
    #[error("error while parsing request to string! {}", self)]
    ParsingRequestToStringErr(#[from] std::string::FromUtf8Error)
}
// impl From<core::convert::Infallible> for RhttpError {
//     fn from(value: core::convert::Infallible) -> Self {
//        Self::ParsingContentLength(value)
//     }
// }
//
// impl From<FromUtf8Error> for RhttpError {
//     fn from(value: FromUtf8Error) -> Self {
//         Self::ParsingRequestErr
//     }
// }
//
// impl From<Utf8Error> for RhttpError {
//     fn from(value: Utf8Error) -> Self {
//         Self::HeaderValueErr(value)
//     }
// }
