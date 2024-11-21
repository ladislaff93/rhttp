#![warn(clippy::pedantic)]
use core::{app::App, from_request::{PathParam, QueryParams}};
use std::{fs, path::Path};
use http::method::Method;
use serde::Deserialize;


fn handle_post_base() -> String {
    String::new()
}

fn handle_post_empty_reply() {
}

fn handle_get_base() -> String {
    let path_to_hello = Path::new("./core/src/hello.html");
    fs::read_to_string(path_to_hello).unwrap()
}

#[derive(Deserialize)]
struct S {
    order_id:usize,
    activity_id:usize
}

fn handle_get_query_params(QueryParams(s):QueryParams<S>) -> String {
    let order_id = s.order_id;
    let activity_id = s.activity_id;
    format!("order_id: {order_id}, activity_id: {activity_id}")
}

fn handle_get_path_params(PathParam(order_id): PathParam<usize>) -> String {
    format!("order_id: {order_id}")
}

fn handle_get_path_params_with_two(PathParam(order_id): PathParam<usize>, PathParam(activity_id): PathParam<usize>) -> String {
    format!("order_id: {order_id}, activity_id: {activity_id}")
}

fn main() {
    let mut app = App::new();
    app.bind_address("127.0.0.1:8080");

    // register handlers
    app.register_path(Method::Get, "/", handle_get_base);
    app.register_path(Method::Get, "/order/{order_id}", handle_get_path_params);
    app.register_path(Method::Get, "/order/{order_id}/activity/{activity_id}", handle_get_path_params_with_two);
    app.register_path(Method::Get, "/path", handle_get_query_params);
    app.register_path(Method::Get, "/empty", handle_post_empty_reply);
    app.register_path(Method::Post, "/", handle_post_base);

    app.listen();
}
