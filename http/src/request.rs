use std::collections::BTreeMap;
use crate::{common::RhttpError, headers::{HeaderType, HeaderValue}, method::Method, version::ProtocolVersion};

#[derive(Debug, Default, Clone)]
pub struct Request {
    pub request_line : RequestLine,
    pub headers: BTreeMap<HeaderType, HeaderValue>,
    pub body: String
}

#[derive(Debug, Default, Clone)]
pub struct RequestLine {
    pub method: Method,
    pub path: String,
    pub protocol_version: ProtocolVersion,
}

impl Request {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_method(&mut self, method: Method) {
        self.request_line.method = method;
    }

    pub fn add_path(&mut self, path: String) {
        self.request_line.path = path;
    }

    pub fn add_protocol_version(&mut self, protocol_version: ProtocolVersion) {
        self.request_line.protocol_version = protocol_version;
    }

    pub fn add_header(&mut self, key: String, val: String) -> Result<(), RhttpError> {
        let header_type = HeaderType::from_string(key)?;
        let header_value = val.parse::<HeaderValue>()?;
        self.headers.insert(header_type, header_value);
        Ok(())
    }
}
