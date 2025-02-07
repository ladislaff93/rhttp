use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ProtocolVersion {
    Http10,
    #[default] 
    Http11
}

impl ProtocolVersion {
    pub fn parse_from_str(protocol: &str) -> Self {
        match protocol.to_uppercase().as_str() {
            "HTTP/1.0" => Self::Http10,
            "HTTP/1.1" => Self::Http11,
            _ => unreachable!("Any other http protocol does not exists")
        } 
    }
}

impl Display for ProtocolVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Self::Http10 => "HTTP/1.0",
            Self::Http11 => "HTTP/1.1"
        };
        write!(f, "{}", v)
    }
}
