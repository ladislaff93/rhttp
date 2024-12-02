pub mod router;
pub mod handler;
pub mod from_request;
pub mod endpoint;
pub mod incoming;
pub mod outcoming;
pub mod url;


#[cfg(test)]
mod tests {
    // use proc_macros::get;
}

// Received data: HttpRequest { method: "GET", path: "/", protocol_version: "HTTP/1.1" }
// Received data: "GET / HTTP/1.1"
// Received data: "Host: 127.0.0.1:8080"
// Received data: "Connection: keep-alive"
// Received data: "Cache-Control: max-age=0"
// Received data: "sec-ch-ua: \"Not)A;Brand\";v=\"99\", \"Google Chrome\";v=\"127\", \"Chromium\";v=\"127\""
// Received data: "sec-ch-ua-mobile: ?0"
// Received data: "sec-ch-ua-platform: \"Linux\""
// Received data: "Upgrade-Insecure-Requests: 1"
// Received data: "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/127.0.0.0 Safari/537.36"
// Received data: "Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7"
// Received data: "Sec-Fetch-Site: none"
// Received data: "Sec-Fetch-Mode: navigate"
// Received data: "Sec-Fetch-User: ?1"
// Received data: "Sec-Fetch-Dest: document"
// Received data: "Accept-Encoding: gzip, deflate, br, zstd"
// Received data: "Accept-Language: sk-SK,sk;q=0.9,cs;q=0.8,en-US;q=0.7,en;q=0.6,ru;q=0.5"

/* 
TODO:
    - Implements MVP generic handlers that will take 0 to n arguments and return impl IntoResponse trait | DONE
    - implements response
    - implement url parsing
    - implements other part of the http protocol
    - headers processing impl:
        - Process all request headers, headers enum and parsing
        - header type
        - header value
        - conversions
        - pretty printing headers map
    -add async implementation
    -add logging tracing
    -db support
    -macro for creating handlers   
*/
