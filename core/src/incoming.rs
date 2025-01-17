use http::{
    common::RhttpError,
    common::{
        RhttpError::{
            ParsingHttpMethodErr, ParsingHttpProtocolErr, ParsingPathErr, ParsingRequestErr,
        },
        CRLF, FINAL_CRLF,
    },
    method::Method,
    request::Request,
    version::ProtocolVersion,
};

#[derive(Debug, Default, Clone)]
pub struct Incoming {
    pub request: Request,
    pub query_params: String,
    pub path_params: Vec<String>,
}

impl Incoming {
    pub fn from(request_line: String) -> Result<Self, RhttpError> {
        let mut incoming = Self::default();
        let (first_line, other) = request_line
            .split_once(CRLF)
            .ok_or(ParsingRequestErr(String::new()))?;
        let (request_headers, request_body) = other
            .split_once(FINAL_CRLF)
            .ok_or(ParsingRequestErr(String::new()))?;
        incoming.parse_request_line(first_line.to_owned())?;
        incoming.parse_headers(request_headers.to_owned());
        incoming.request.add_body(request_body.to_owned());
        Ok(incoming)
    }

    pub fn set_path_params(&mut self, path_params: Vec<String>) {
        self.path_params = path_params
    }

    pub fn get_request_method(&self) -> &Method {
        &self.request.request_line.method
    }

    pub fn get_request_path(&self) -> &str {
        &self.request.request_line.path
    }

    fn parse_request_line(&mut self, request_line: String) -> Result<(), RhttpError> {
        let mut parts = request_line.split_whitespace();

        let method = Method::from_str(parts.next().ok_or(ParsingHttpMethodErr)?);
        self.request.add_method(method);

        let path = parts.next().ok_or(ParsingPathErr)?;

        if let Some((path, params)) = path.split_once("?") {
            self.query_params = params.to_owned();
            self.request.add_path(path.to_owned());
        } else {
            self.request.add_path(path.to_owned());
        }

        let protocol_version =
            ProtocolVersion::from_str(parts.next().ok_or(ParsingHttpProtocolErr)?);
        self.request.add_protocol_version(protocol_version);

        Ok(())
    }

    fn parse_headers(&mut self, request_headers: String) {
        request_headers.split(CRLF).for_each(|headers| {
            let (key, value) = headers
                .split_once(": ")
                .expect("properly formatted header item");
            self.request.add_header(key.to_owned(), value.to_owned());
        });
    }
}
