use std::collections::HashMap;
use crate::{headers::{HeaderType, HeaderValue}, method::Method, version::ProtocolVersion};

#[derive(Debug, Default, Clone)]
pub struct Request {
    pub request_line : RequestLine,
    pub headers: HashMap<HeaderType, HeaderValue>,
    pub body: String
}

#[derive(Debug, Default, Clone)]
pub struct RequestLine {
    pub method: Method,
    pub path: String,
    pub protocol_version: ProtocolVersion,
}

// impl <'r> Default for RequestLine<'r> {
//     fn default() -> Self {
//         Self {
//             method: Method::default(),
//             path: "",
//             protocol_version: ProtocolVersion::default()
//         }
//     }
// }

// impl <'r> Default for Request<'r> {
//     fn default() -> Self {
//         Self {
//             request_line: RequestLine::default(),
//             headers: HashMap::default(),
//             body: ""
//         }
//     }
// }

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

    pub fn add_header(&mut self, key: String, val: String) {
        let header_type = HeaderType::from_string(key).expect("valid header type item");
        let header_value = val.parse::<HeaderValue>().unwrap();
        self.headers.entry(header_type).or_insert(header_value);
    }

    pub fn add_body(&mut self, body: String) {
        self.body = body;
    }

}
