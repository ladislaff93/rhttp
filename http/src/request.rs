use std::collections::HashMap;
use crate::{headers::{HeaderType, HeaderValue}, method::Method, version::ProtocolVersion};

#[derive(Debug, Default)]
pub struct Request<'r> {
    pub request_line : RequestLine<'r>,
    pub headers: HashMap<HeaderType<'r>, HeaderValue>,
    pub body: &'r str
}

#[derive(Debug, Default)]
pub struct RequestLine<'r> {
    pub method: Method,
    pub path: &'r str,
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

impl <'r> Request<'r> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_method(&mut self, method: Method) {
        self.request_line.method = method;
    }

    pub fn add_path(&mut self, path: &'r str) {
        self.request_line.path = path;
    }

    pub fn add_protocol_version(&mut self, protocol_version: ProtocolVersion) {
        self.request_line.protocol_version = protocol_version;
    }

    pub fn add_header(&mut self, key: &'r str, val: &'r str) {
        let header_type = HeaderType::from_str(key).expect("valid header type item");
        let header_value = val.parse::<HeaderValue>().unwrap();
        self.headers.entry(header_type).or_insert(header_value);
    }

    pub fn add_body(&mut self, body: &'r str) {
        self.body = body;
    }

}
