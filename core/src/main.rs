use core::{from_request::{PathParam, QueryParams}, router::Router};
use http::{method::Method, response::Html};
use serde::Deserialize;

async fn handle_post_base() -> String {
    String::new()
}

async fn handle_post_empty_reply() {
}

async fn handle_get_base() -> Html {
    Html(std::include_str!("../../core/src/hello.html").to_string())
}

#[derive(Deserialize)]
struct S {
    order_id:usize,
    activity_id:usize
}

async fn handle_get_query_params(QueryParams(s):QueryParams<S>) -> String {
    let order_id = s.order_id;
    let activity_id = s.activity_id;
    format!("order_id: {order_id}, activity_id: {activity_id}")
}

async fn handle_get_path_params(PathParam(order_id): PathParam<usize>) -> String {
    format!("order_id: {order_id}")
}

async fn handle_get_path_params_with_two(PathParam(order_id): PathParam<usize>, PathParam(activity_id): PathParam<usize>) -> String {
    format!("order_id: {order_id}, activity_id: {activity_id}")
}

#[async_std::main]
async fn main() {
    let mut app = Router::new();
    app.bind_address("127.0.0.1:8080").await.unwrap();

    // register handlers
    app.register_path(Method::Get, "/", handle_get_base);
    app.register_path(Method::Get, "/order/:order_id", handle_get_path_params);
    app.register_path(Method::Get, "/order/:order_id/activity/:activity_id", handle_get_path_params_with_two);
    app.register_path(Method::Get, "/path", handle_get_query_params);
    app.register_path(Method::Get, "/empty", handle_post_empty_reply);
    app.register_path(Method::Post, "/", handle_post_base);

    app.listen().await.unwrap();
}
