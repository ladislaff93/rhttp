use std::collections::HashMap;
use serde_json::Value;
use crate::{common::{RhttpErr, CRLF, FINAL_CRLF}, headers::{HeaderType, HeaderValue}, method::Method, version::ProtocolVersion};

#[derive(Debug)]
pub struct Request<'r> {
    pub method: Method,
    pub path: &'r str,
    pub protocol_version: ProtocolVersion,
    pub query_params: &'r str,
    pub path_params: Vec<&'r str>,
    pub headers: HashMap<HeaderType<'r>, HeaderValue>,
    pub body: Option<&'r str>
}

impl <'r> Default for  Request<'r> {
    fn default() -> Self {
        Self {
            method: Method::default(),
            path: "",
            protocol_version: ProtocolVersion::default(),
            query_params: "",
            path_params: Vec::default(),
            headers: HashMap::default(),
            body: None
        }
    }
}

impl <'r> Request<'r> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_parts(request_line: &'r str) -> Result<Self, RhttpErr> {
        println!("RAW r: {}", request_line);
        println!("----------------------");
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

        self.method = Method::from_str(
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
                let header_value: HeaderValue = value.parse().unwrap();
                self.headers.entry(header_type).or_insert(header_value);
            });
    }

    fn parse_body(&mut self, request_body: &'r str) {
        if let Some(length) = self.headers.get(&HeaderType::ContentLength) {
            if length.to_str().unwrap().parse::<usize>().unwrap() != 0 {
                self.body = Some(request_body);
                let v: Value = serde_json::from_str(self.body.as_ref().unwrap()).unwrap();
                println!("Request body: {:#?}", v);
            }
        }
    }

    pub fn add_path_params(&mut self, path_params: Vec<&'r str>) {
        self.path_params = path_params
    }
}
