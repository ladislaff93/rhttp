use http::{common::{RhttpErr, CRLF, FINAL_CRLF}, headers::HeaderType, method::Method, request::Request, version::ProtocolVersion};
use serde_json::Value;

#[derive(Debug, Default, Clone)]
pub struct Incoming<'r> {
    pub request: Request<'r>,
    pub query_params: &'r str,
    pub path_params: Vec<&'r str>,
}

impl <'r> Incoming<'r> {
    pub fn from(request_line: &'r str) -> Result<Self, RhttpErr> {
        println!("RAW r: {}", request_line);
        println!("----------------------");
        let mut incoming = Self::default();
        let (first_line, other) = request_line.split_once(CRLF).ok_or(RhttpErr::ParsingRequestErr)?;
        let (request_headers, request_body) = other.split_once(FINAL_CRLF).ok_or(RhttpErr::ParsingRequestErr)?;
        incoming.parse_request_line(first_line)?;
        incoming.parse_headers(request_headers);
        incoming.parse_body(request_body);
        Ok(incoming)
    }

    pub fn set_query_params(&mut self, query_params: &'r str) {
        self.query_params = query_params
    }

    pub fn set_path_params(&mut self, path_params: Vec<&'r str>) {
        self.path_params = path_params
    }

    pub fn get_request_method(&self) -> Method {
        self.request.request_line.method
    }

    pub fn get_request_path(&self) -> &str {
        self.request.request_line.path
    }

    fn parse_request_line(&mut self, request_line: &'r str) -> Result<(), RhttpErr> {
        let mut parts = request_line.split_whitespace();

        let method = Method::from_str(
            parts.next()
                .ok_or(RhttpErr::ParsingHttpMethodErr)?
        );
        self.request.add_method(method);

        let path = parts.next()
            .ok_or(RhttpErr::ParsingPathErr)?;

        if let Some((path, params)) = path.split_once("?") {
            self.query_params = params;
            self.request.add_path(path);
        } else {
            self.request.add_path(path);
        }

        let protocol_version = ProtocolVersion::from_str(
            parts.next()
                .ok_or(RhttpErr::ParsingHttpProtocolErr)?
        );
        self.request.add_protocol_version(protocol_version);

        Ok(())
    }

    fn parse_headers(&mut self, request_headers: &'r str) {
        request_headers.split(CRLF)
            .for_each(|headers| {
                let (key, value) = headers.split_once(": ").expect("properly formatted header item"); 
                self.request.add_header(key, value);
            });
    }

    fn parse_body(&mut self, request_body: &'r str) {
        if let Some(length) = self.request.headers.get(&HeaderType::ContentLength) {
            if length.to_str().unwrap().parse::<usize>().unwrap() != 0 {
                self.request.add_body(request_body);
                let v: Value = serde_json::from_str(self.request.body).unwrap();
                println!("Request body: {:#?}", v);
            }
        }
    }
}
