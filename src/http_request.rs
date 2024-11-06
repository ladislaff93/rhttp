use std::collections::HashMap;
use serde_json::Value;
use crate::common::{RhttpErr, CRLF, FINAL_CRLF};
use crate::headers::HeaderType;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum HttpMethod {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Trace,
    Connect,
    Patch,
    #[default] None
}

impl HttpMethod {
    pub fn from_str(method: &str) -> Self {
        match method.to_uppercase().as_str() {
            "GET" => Self::Get,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "OPTIONS" => Self::Options,
            "HEAD" => Self::Head,
            "TRACE" => Self::Trace,
            "CONNECT" => Self::Connect,
            "PATCH" => Self::Patch,
            _ => unreachable!("Any other http request method does not exists"),

        }
    }

    pub fn iterator() -> std::slice::Iter<'static, HttpMethod> {
        static HTTP_METHOD: [HttpMethod;9] = [
            HttpMethod::Get,
            HttpMethod::Put,
            HttpMethod::Post,
            HttpMethod::Delete,
            HttpMethod::Options,
            HttpMethod::Head,
            HttpMethod::Trace,
            HttpMethod::Connect,
            HttpMethod::Patch,
        ];
        HTTP_METHOD.iter()
    }
}

pub enum Path {

}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ProtocolVersion {
    #[default] Http1
}

impl ProtocolVersion {
    pub fn from_str(protocol: &str) -> Self {
        let (first, _) = protocol.split_once(".").unwrap();
        match first.to_uppercase().as_str() {
            "HTTP/1" => Self::Http1,
            _ => unreachable!("Any other http protocol does not exists")
        } 
    }
}

#[derive(Debug)]
pub struct HttpRequest<'r> {
    pub method: HttpMethod,
    pub path: &'r str,
    pub protocol_version: ProtocolVersion,
    pub query_params: &'r str,
    pub path_params: Vec<&'r str>,
    pub headers: HashMap<HeaderType<'r>, &'r str>,
    pub content_length: usize,
    pub body: Option<&'r str>
}

impl <'r> Default for  HttpRequest<'r> {
    fn default() -> Self {
        Self {
            method: HttpMethod::default(),
            path: "",
            protocol_version: ProtocolVersion::default(),
            query_params: "",
            path_params: Vec::default(),
            headers: HashMap::default(),
            content_length: usize::default(),
            body: None
        }
    }
}

impl <'r> HttpRequest<'r> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_parts(request_line: &'r str) -> Result<Self, RhttpErr> {
        let mut new_request = Self::new();
        let (request_line, other) = request_line.split_once(CRLF).ok_or(RhttpErr::ParsingRequestErr)?;
        let (request_headers, request_body) = other.split_once(FINAL_CRLF).ok_or(RhttpErr::ParsingRequestErr)?;
        new_request.parse_request_line(request_line)?;
        new_request.parse_headers(request_headers);
        new_request.parse_body(request_body);
        Ok(new_request)
    }

    fn parse_request_line(&mut self, request_line: &'r str) -> Result<(), RhttpErr> {
        let mut parts = request_line.split_whitespace();

        self.method = HttpMethod::from_str(
            parts.next()
                .ok_or(RhttpErr::ParsingHttpMethodErr)?
        );

        let path = parts.next()
            .ok_or(RhttpErr::ParsingPathErr)?;

        if let Some((path, params)) = path.split_once("?") {
            self.query_params = params;
            self.path = path;
        } else {
            self.query_params = "";
            self.path = path;
        }

        self.protocol_version = ProtocolVersion::from_str(
            parts.next()
                .ok_or(RhttpErr::ParsingHttpProtocolErr)?
        );
        Ok(())
    }

    fn parse_headers(&mut self, request_headers: &'r str) {
        request_headers.split(CRLF)
            .for_each(|headers| {
                let (key, value) = headers.split_once(": ").expect("properly formatted header item"); 
                let header_type = HeaderType::from_str(key).expect("valid header type item");
                // if key == "Content-Length" {
                //     self.content_length = value.parse()
                //         .expect("correct number for content length");
                // }
                self.headers.entry(header_type).or_insert(value);
            });
    }

    fn parse_body(&mut self, request_body: &'r str) {
        if self.content_length > 0 {
            self.body = Some(request_body);
            let v: Value = serde_json::from_str(self.body.as_ref().unwrap()).unwrap();
            println!("Request body: {:#?}", v);
        }
    }

    pub fn add_path_params(&mut self, path_params: Vec<&'r str>) {
        self.path_params = path_params
    }
}
