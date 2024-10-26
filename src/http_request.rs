use std::{collections::{HashMap, VecDeque}, io::{BufReader, Write}, str::from_utf8};
use std::io::Read;
use std::io::BufRead;
use serde_json::Value;


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

#[derive(Debug)]
pub struct HttpRequest<'r> {
    pub method: HttpMethod,
    pub path: &'r str,
    pub protocol_version: &'r str,
    pub query_params: &'r str,
    pub path_params: Vec<&'r str>,
    pub headers: HashMap<&'r str, &'r str>,
    pub content_length: usize,
    pub body: Option<&'r str>
}

impl <'r> Default for  HttpRequest<'r> {
    fn default() -> Self {
        Self {
            method: HttpMethod::default(),
            path: "",
            protocol_version: "",
            query_params: "",
            path_params: Vec::default(),
            headers: HashMap::default(),
            content_length: usize::default(),
            body: None
        }
    }
}

impl <'r> HttpRequest<'r> {
    pub fn new<S: Read + Write>(stream: S) -> Self {
        let mut http_request = HttpRequest::default();

        let mut buf_reader = BufReader::new(stream);
        let mut request_parts = Self::load_request(&mut buf_reader);

        let first_line = request_parts.pop_front().unwrap();
        let mut request_line = first_line.split_whitespace();

        let method_str = request_line.next().expect("method in first part of request line");
        let path = request_line.next().expect("path in second part of request line");

        if let Some((path, params)) = path.split_once("?") {
            http_request.query_params = params;
            http_request.path = path;
        } else {
            http_request.query_params = Default::default();
            http_request.path = path;
        }

        http_request.protocol_version = request_line.next().expect("protocol version in third part of request line");
        http_request.method = HttpMethod::from_str(method_str);

         
        Self::parse_headers(&mut http_request, request_parts);
        Self::parse_body(&mut http_request, &mut buf_reader);

        http_request
    }

    fn parse_headers(http_request: &mut Self, request_parts: VecDeque<String>) {
        for i in request_parts {
            let mut split = i.splitn(2, ": ");
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                if key == "Content-Length" {
                   http_request.content_length =  value.parse().unwrap();
                }
                http_request.headers.entry(key).or_insert(value);
            }
        }
    }

    fn parse_body<B: Read + Write>(http_request: &mut Self, buf_reader: &mut BufReader<B>) {
        if http_request.content_length > 0 {
            let mut buff = vec![0_u8; http_request.content_length];
            let _ = &buf_reader.read_exact(&mut buff).unwrap();
            http_request.body = Some(from_utf8(&buff).unwrap());
            let v: Value = serde_json::from_str(http_request.body.as_ref().unwrap()).unwrap();
            println!("Request body: {:#?}", v);
        }
    }

    fn load_request(buf_reader: &mut BufReader<impl Read + Write>) -> VecDeque<String> {
        buf_reader.by_ref()
            .lines()
            .map(|res| res.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<VecDeque<String>>()
    }

    pub fn add_path_params(&mut self, path_params: Vec<&'r str>) {
        self.path_params = path_params
    }
}
