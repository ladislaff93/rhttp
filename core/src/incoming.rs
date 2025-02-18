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
    pub wildcard_param: String,
}

impl Incoming {
    pub(crate) fn from(request_line: &str) -> Result<Self, RhttpError> {
        let mut incoming = Self::default();
        let (first_line, other) = request_line
            .split_once(CRLF)
            .ok_or(ParsingRequestErr(String::new()))?;
        let (request_headers, request_body) = other
            .split_once(FINAL_CRLF)
            .ok_or(ParsingRequestErr(String::new()))?;
        incoming.parse_request_line(first_line)?;
        incoming.parse_headers(request_headers);
        request_body.clone_into(&mut incoming.request.body);
        Ok(incoming)
    }

    pub(crate) fn get_request_method(&self) -> &Method {
        &self.request.request_line.method
    }

    pub(crate) fn get_request_path(&self) -> &str {
        &self.request.request_line.path
    }

    fn parse_request_line(&mut self, request_line: &str) -> Result<(), RhttpError> {
        let mut parts = request_line.split_whitespace();

        let method = Method::parse_from_str(parts.next().ok_or(ParsingHttpMethodErr)?);
        self.request.add_method(method);

        let path = parts.next().ok_or(ParsingPathErr)?;

        if let Some((path, params)) = path.split_once('?') {
            params.clone_into(&mut self.query_params);
            self.request.add_path(path.to_owned());
        } else {
            self.request.add_path(path.to_owned());
        }

        self.request
            .add_protocol_version(ProtocolVersion::parse_from_str(
                parts.next().ok_or(ParsingHttpProtocolErr)?,
            ));

        Ok(())
    }

    fn parse_headers(&mut self, request_headers: &str) {
        request_headers.split(CRLF).for_each(|headers| {
            let (key, value) = headers
                .split_once(": ")
                .expect("properly formatted header item");
            self.request
                .add_header(key.to_owned(), value.to_owned())
                .expect("");
        });
    }
}
