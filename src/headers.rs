use std::str::FromStr;

use crate::common::RhttpErr;

struct Header<'r>(HeaderType<'r>, &'r str);

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HeaderType<'r> {
    Standard(Standard),
    Custom(&'r str)
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Standard {
    /// The HTTP Accept request header indicates which content types, expressed 
    /// as MIME types, the client is able to understand. 
    ///
    /// The server uses content negotiation to select one of the proposals and 
    /// informs the client of the choice with the Content-Type response header.
    /// Browsers set required values for this header based on the context of the
    /// request. For example, a browser uses different values in a request when 
    /// fetching a CSS stylesheet, image, video, or a script.
    Accept,
    /// The Host request header specifies the host and port number of the server
    /// to which the request is being sent.
    /// 
    /// If no port is included, the default port for the service requested is 
    /// implied (e.g., 443 for an HTTPS URL, and 80 for an HTTP URL).
    /// A Host header field must be sent in all HTTP/1.1 request messages.
    ///
    /// A 400 (Bad Request) status code may be sent to any HTTP/1.1 request 
    /// message that lacks or contains more than one Host header field. 
    /// Host, The HTTP Connection header controls whether the network connection
    /// stays open after the current transaction finishes.
    ///
    /// If the value sent is keep-alive, the connection is persistent and not 
    /// closed, allowing subsequent requests to the same server on the same 
    /// connection.
    Connection,
    /// The HTTP Content-Length header indicates the size, in bytes, of the
    /// message body sent to the recipient.
    ContentLength,
    /// The HTTP Content-Type representation header is used to indicate the 
    /// original media type of a resource before any content encoding is applied.
    ///
    /// In responses, the Content-Type header informs the client about the media
    /// type of the returned data. In requests such as POST or PUT, the client 
    /// uses the Content-Type header to specify the type of content being sent 
    /// to the server. 
    ///
    /// If a server implementation or configuration is strict 
    /// about content type handling, a 415 client error response may be returned.
    /// The Content-Type header differs from Content-Encoding in that Content-Encoding
    /// helps the recipient understand how to decode data to its original form.
    ContentType,
    /// The HTTP Accept-Encoding request header indicates the content encoding 
    /// (usually a compression algorithm) that the client can understand. 
    ///
    /// The server uses content negotiation to select one of the proposals and 
    /// informs the client of that choice with the Content-Encoding response header.
    ///
    /// Even if both the client and the server support the same compression 
    /// algorithms, the server may choose not to compress the body of a response
    /// if the identity value is also acceptable. This happens in two common cases:
    /// 
    /// The data is already compressed, meaning a second round of compression 
    /// will not reduce the transmitted data size, and may actually increase the
    /// size of the content in some cases. This is true for pre-compressed 
    /// image formats (JPEG, for instance).
    ///
    /// The server is overloaded and cannot allocate computing resources to 
    /// perform the compression. For example, Microsoft recommends not to 
    /// compress if a server uses more than 80% of its computational power.
    ///
    /// As long as the identity;q=0 or *;q=0 directives do not explicitly forbid
    /// the identity value that means no encoding, the server must never return 
    /// a 406 Not Acceptable error.
    AcceptEncoding,
    /// The User-Agent request header is a characteristic string that lets 
    /// servers and network peers identify the application, operating system,
    /// vendor, and/or version of the requesting user agent. 
    UserAgent
}

impl FromStr for Standard {
    type Err = RhttpErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Accept" => Ok(Self::Accept),
            "Connection" => Ok(Self::Connection),
            "Content-Length" => Ok(Self::ContentLength),
            "Content-Type" => Ok(Self::ContentType),
            "Accept-Encoding" => Ok(Self::AcceptEncoding),
            "User-Agent" => Ok(Self::UserAgent),
            _ => Err(RhttpErr::ParsingHttpHeaderErr),
        }
    }
}

impl <'r> HeaderType<'r> {
    pub fn from_str(s: &'r str) -> Result<Self, RhttpErr> {
        if let Ok(std_header) = Standard::from_str(s) {
            return Ok(Self::Standard(std_header));
        } else {
            // TODO: add validation of Custom header 
            // this is always true but i just created bare bone structure for future validation
            if let result = Ok(Self::Custom(s)) {
                return result;
            }
            Err(RhttpErr::ParsingHttpHeaderErr)
        }
    }
}

