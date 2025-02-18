// #![deny(clippy::unwrap_used, clippy::panic, clippy::cognitive_complexity)]
// #![warn(clippy::pedantic, clippy::complexity)]
// #![allow(clippy::manual_async_fn, clippy::must_use_candidate, clippy::missing_panics_doc, clippy::missing_errors_doc, clippy::too_many_lines)]
#![warn(
    clippy::all,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::mem_forget,
    clippy::unused_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    clippy::match_wildcard_for_single_variants,
    clippy::if_let_mutex,
    clippy::await_holding_lock,
    clippy::match_on_vec_items,
    clippy::imprecise_flops,
    clippy::suboptimal_flops,
    clippy::lossy_float_literal,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::fn_params_excessive_bools,
    clippy::exit,
    clippy::inefficient_to_string,
    clippy::linkedlist,
    clippy::macro_use_imports,
    clippy::option_option,
    clippy::verbose_file_reads,
    clippy::unnested_or_patterns,
    clippy::str_to_string,
    rust_2018_idioms,
    future_incompatible,
    nonstandard_style,
    missing_debug_implementations,
    // missing_docs
)]
#![deny(unreachable_pub)]
#![allow(elided_lifetimes_in_paths, clippy::type_complexity, clippy::manual_async_fn)]
#![forbid(unsafe_code)]

use http::method::Method;
pub mod endpoint;
pub mod from_request;
pub mod handler;
pub mod incoming;
pub mod radix_tree;
pub mod router;

#[derive(Hash)]
pub(crate) struct EndpointId {
    method: Method,
    path: &'static str,
}

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
    - efficient storing and prasing of http request (use bytes::Bytes and slice windows into data instead of owning values)
    - macro for creating handlers
    - implement url parsing
    - implements other part of the http protocol
    - headers processing impl:
        - Process all request headers, headers enum and parsing
        - header type
        - header value
        - conversions
*/
