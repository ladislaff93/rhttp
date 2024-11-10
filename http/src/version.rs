#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Default)]
pub enum ProtocolVersion {
    #[default] Http1
}

impl ProtocolVersion {
    pub fn from_str(protocol: &str) -> Self {
        let (first, _) = protocol.split_once(".").unwrap();
        match first.to_uppercase().as_str() {
            "HTTP/1" => Self::Http1,
            _ => unreachable!("Any other http protocol does not exists")
        } 
    }
}
