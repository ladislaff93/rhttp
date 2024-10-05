use std::{collections::{HashMap, VecDeque}, io::BufReader, net::TcpStream, str::from_utf8};
use std::io::Read;
use std::io::BufRead;
use serde_json::Value;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
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
}

impl HttpMethod {
    fn from_str(method: &str) -> Self {
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
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[derive(Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub protocol_version: String,
    pub query_params: String,
    pub path_params: Vec<String>,
    pub headers: HashMap<String, String>,
    pub content_length: usize,
    pub body: Option<String>
}


impl HttpRequest {
    pub fn new(stream: &TcpStream) -> Self {
        let mut headers = HashMap::<String, String>::new();
        let mut buf_reader = BufReader::new(stream);
        let mut http_request = Self::load_request(&mut buf_reader);
        let mut content_length = 0;
        let mut body = None;
        let url;
        let query_params;

        // parse request line
        let rl = &mut http_request.pop_front().unwrap();
        let mut request_line = rl.split_whitespace();
        let method_str = request_line.next().expect("method in first part of request line");
        let path = request_line.next().expect("path in second part of request line").to_string();
        if let Some((u, params)) = path.split_once("?") {
            query_params = params.to_string();
            url=u.to_string();
        } else {
            url = path;
            query_params = Default::default();
        }
        // empty query params
        let protocol_version = request_line.next().expect("protocol_version in third part of request line").to_string();
        let method = HttpMethod::from_str(method_str);

        // parse path params
        // /hello/4/where/5
         
        // parse headers
        for i in http_request {
            let mut split = i.splitn(2, ": ");
            if let (Some(key), Some(value)) = (split.next(), split.next()) {
                if key == "Content-Length" {
                   content_length =  value.parse().unwrap();
                }
                headers.entry(key.to_string()).or_insert(value.to_string());
            }
        }
        
        // load body if exists
        if content_length > 0 {
            let mut buff = vec![0_u8; content_length];
            let _ = &buf_reader.read_exact(&mut buff).unwrap();
            body = Some(from_utf8(&buff).unwrap().to_string());
            let v: Value = serde_json::from_str(body.as_ref().unwrap()).unwrap();
            println!("Request body: {:#?}", v);
        }

        Self {
            method,
            path: url.to_string(),
            protocol_version,
            query_params,
            path_params: Vec::new(),
            headers,
            content_length,
            body
        }
    }

    fn parse_headers() {
        todo!();
    }

    fn parse_body() {
        todo!();
    }

    fn load_request(buf_reader: &mut BufReader<&TcpStream>) -> VecDeque<String> {
        buf_reader.by_ref()
            .lines()
            .map(|res| res.unwrap())
            .take_while(|line| !line.is_empty())
            .collect::<VecDeque<String>>()
    }
}
