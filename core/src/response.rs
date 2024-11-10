#[derive(Debug, Eq, PartialEq)]
pub struct Response {
    pub body: String
}

pub trait IntoResponse {
    fn into_response(&self) -> Response;
}

impl IntoResponse for &str {
    fn into_response(&self) -> Response {
        Response{
            body: self.to_string()
        }
    }
}

impl IntoResponse for String {
    fn into_response(&self) -> Response {
        Response{
            body: self.to_string()
        }
    }
}
