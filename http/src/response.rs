use crate::{
    common::{RhttpError, CRLF},
    headers::{HeaderType, HeaderValue},
    status_code::Status,
    version::ProtocolVersion,
};
use bytes::{BufMut, BytesMut};
use chrono::Utc;
use std::collections::BTreeMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Response<'rs> {
    pub status_line: StatusLine<'rs>,
    pub headers: BTreeMap<HeaderType, HeaderValue>,
    pub body: String,
}

#[derive(Debug, Eq, PartialEq)]
pub struct StatusLine<'rs> {
    pub version: ProtocolVersion,
    pub status_code: usize,
    pub reason_phrase: &'rs str,
}

impl<'rs> Default for StatusLine<'rs> {
    fn default() -> Self {
        Self {
            version: ProtocolVersion::default(),
            status_code: Status::default().status_code(),
            reason_phrase: Status::default().as_str(),
        }
    }
}

impl<'rs> Default for Response<'rs> {
    fn default() -> Self {
        let mut zelf = Self {
            status_line: StatusLine::default(),
            headers: BTreeMap::default(),
            body: String::default(),
        };
        zelf.add_header(
            HeaderType::Date,
            Utc::now().format("%a, %d %b %Y %H:%M:%S GMT").to_string(),
        );
        zelf
    }
}

pub trait IntoResponse {
    fn into_response<'rs>(self) -> Response<'rs>;
}

pub struct Html(pub String);

impl<'rs> Response<'rs> {
    fn add_header<T>(&mut self, key: HeaderType, val: T)
    where
        T: TryInto<HeaderValue, Error = RhttpError>,
    {
        let header_value = val.try_into().unwrap();
        self.headers.entry(key).or_insert(header_value);
    }

    fn add_status(&mut self, status: Status) {
        self.status_line.status_code = status.status_code();
        self.status_line.reason_phrase = status.as_str();
    }

    pub fn serialize(&self) -> BytesMut {
        let mut return_buff = BytesMut::new();

        return_buff.put(
            format!(
                "{} {} {}{CRLF}",
                self.status_line.version,
                self.status_line.status_code,
                self.status_line.reason_phrase
            )
            .as_bytes(),
        );
        self.headers.iter().for_each(|(header_type, header_value)| {
            return_buff
                .put(format!("{}: {}{}", header_type, header_value.to_str(), CRLF).as_bytes())
        });
        return_buff.put(CRLF.as_bytes());
        return_buff.put(self.body.as_bytes());

        return_buff
    }
}

impl IntoResponse for Html {
    fn into_response<'rs>(self) -> Response<'rs> {
        let mut resp = Response {
            body: self.0,
            ..Response::default()
        };
        resp.add_header(HeaderType::ContentType, mime::TEXT_HTML_UTF_8.as_ref());
        resp
    }
}

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

impl<T> IntoResponse for (Status, T)
where
    T: IntoResponse,
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
            headers: BTreeMap::default(),
            body: self.to_string(),
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
            headers: BTreeMap::default(),
            body: self.to_string(),
        };
        resp.add_header(HeaderType::ContentLength, self.len());
        resp.add_header(HeaderType::ContentType, mime::TEXT_PLAIN_UTF_8.as_ref());

        resp
    }
}
