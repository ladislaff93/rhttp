#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum Method {
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

impl Method {
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
            _ => unreachable!("Any other http request method does not exists")
        }
    }

    pub fn iterator() -> std::slice::Iter<'static, Method> {
        [Method::Get, Method::Put, Method::Post, Method::Delete, Method::Options,
            Method::Head, Method::Trace, Method::Connect, Method::Patch].iter()
    }
}
