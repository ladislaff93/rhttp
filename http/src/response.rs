use std::collections::HashMap;
use crate::{common::RhttpErr, headers::{HeaderType, HeaderValue}, status_code::Status, version::ProtocolVersion};
use chrono::{DateTime, Utc};

#[derive(Debug, Eq, PartialEq)]
pub struct Response<'rs> {
    pub status_line: StatusLine<'rs>,
    pub headers: HashMap<HeaderType<'rs>, HeaderValue>,
    pub body: String
}

#[derive(Debug, Eq, PartialEq)]
pub struct StatusLine<'rs> {
    pub version: ProtocolVersion,
    pub status_code: usize,
    pub reason_phrase: &'rs str
}

impl <'rs> Default for StatusLine<'rs> {
    fn default() -> Self {
        Self {
            version: ProtocolVersion::default(),
            status_code: Status::default().status_code(),
            reason_phrase: Status::default().as_str()
        }
    }
}

impl <'rs> Default for Response<'rs> {
    fn default() -> Self {
        let mut zelf = Self {
            status_line: StatusLine::default(),
            headers: HashMap::default(),
            body: String::default()
        };
        zelf.add_header(HeaderType::Date, Utc::now().format("%a, %d %b %Y %H:%M:%S").to_string() + " GTM");
        zelf
    }
}

pub trait IntoResponse {
    fn into_response<'rs>(self) -> Response<'rs>;
}

pub struct Html<T>(T);

impl <'rs> Response<'rs> {
    fn add_header<T>(&mut self, key: HeaderType<'rs>, val: T)
    where 
        T: TryInto<HeaderValue, Error=RhttpErr>
    {
        let header_value = val.try_into().unwrap();
        self.headers.entry(key).or_insert(header_value);
    }

    fn add_status(&mut self, status: Status) {
        self.status_line.status_code = status.status_code();
        self.status_line.reason_phrase = status.as_str();
    }
}

// impl <T> IntoResponse for Html<T> {
//     fn into_response<'rs>(self) -> Response<'rs> {
//        let resp = Response::default(); 
//     }
// }

impl IntoResponse for () {
    fn into_response<'rs>(self) -> Response<'rs> {
        let mut resp = Response::default();
        resp.add_header(HeaderType::ContentLength, 0);
        resp
    }
}

impl IntoResponse for Status {
    fn into_response<'rs>(self) -> Response<'rs> {
        let mut resp = Response::default();
        resp.add_status(self);
        resp
    }
}

impl <T> IntoResponse for (Status, T) 
where 
    T: IntoResponse 
{
    fn into_response<'rs>(self) -> Response<'rs> {
        let mut resp = self.1.into_response();
        resp.add_status(self.0);
        resp
    }
}

impl IntoResponse for &str {
    fn into_response<'rs>(self) -> Response<'rs> {
        let mut resp = Response {
            status_line: StatusLine::default(),
            headers: HashMap::default(),
            body: self.to_string()
        };
        resp.add_header(HeaderType::ContentLength, self.len());
        resp.add_header(HeaderType::ContentType, mime::TEXT_PLAIN_UTF_8.as_ref());

        resp
         
    }
}

impl IntoResponse for String {
    fn into_response<'rs>(self) -> Response<'rs> {
        let mut resp = Response {
            status_line: StatusLine::default(),
            headers: HashMap::default(),
            body: self.to_string()
        };
        resp.add_header(HeaderType::ContentLength, self.len());
        resp.add_header(HeaderType::ContentType, mime::TEXT_PLAIN_UTF_8.as_ref());

        resp
    }
}
