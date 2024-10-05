use std::{fs, path::Path};
use rhttp::{app::App, common, from_request::QueryParams, http_request::HttpMethod};
use common::{CRLF, FINAL_CRLF};
use serde::Deserialize;


fn handle_post_base() -> String {
    format!("HTTP/1.1 200 OK{FINAL_CRLF}")
}

fn handle_get_base() -> String {
    let path_to_hello = Path::new("./src/hello.html");
    let content = fs::read_to_string(path_to_hello).unwrap();
    let length = content.len();
    format!("HTTP/1.1 200 OK{CRLF}Content-Length: {length}{FINAL_CRLF}{content}")
}

#[derive(Deserialize)]
struct S {
    order_id:usize,
    activity_id:usize
}

fn handle_get_query_params(QueryParams(s):QueryParams<S>) -> String {
    let order_id = s.order_id;
    let activity_id = s.activity_id;
    let content = format!("order_id: {order_id}, activity_id: {activity_id}");
    let length = content.len();
    format!("HTTP/1.1 200 OK{CRLF}Content-Length: {length}{FINAL_CRLF}{content}")
}

fn handle_get_path_params(order_id: usize, activity_id: usize) -> String {
    let content = format!("order_id: {order_id}, activity_id: {activity_id}");
    let length = content.len();
    format!("HTTP/1.1 200 OK{CRLF}Content-Length: {length}{FINAL_CRLF}{content}")
}

fn main() {
    let mut app = App::new();
    app.bind_address("127.0.0.1:8080");

    // register handlers
    app.register_path(HttpMethod::Get, "/", handle_get_base);
    // app.register_path(HttpMethod::Get, "/order/{order_id}/activity/{activity_id}", handle_get_path_params);
    // app.register_path(HttpMethod::Get, "/path", handle_get_path_params);
    app.register_path(HttpMethod::Post, "/", handle_post_base);

    app.listen();
}
