use core::{
    from_request::{PathParam, QueryParams, WildCardParam},
    router::Router,
};
use http::{method::Method, response::Html};
use serde::Deserialize;

async fn handle_post_base(body: String) -> String {
    body
}

async fn handle_post_empty_reply() {}

async fn handle_get_base() -> Html {
    Html(std::include_str!("../../core/src/hello.html").to_string())
}

#[derive(Deserialize)]
struct S {
    order_id: usize,
    activity_id: usize,
}

async fn handle_get_query_params(QueryParams(s): QueryParams<S>) -> String {
    let order_id = s.order_id;
    let activity_id = s.activity_id;
    format!("order_id: {order_id}, activity_id: {activity_id}")
}

async fn handle_get_path_params(PathParam(order_id): PathParam<usize>) -> String {
    format!("order_id: {order_id}")
}

async fn handle_get_path_params_with_two(
    PathParam(order_id): PathParam<usize>,
    PathParam(activity_id): PathParam<usize>,
) -> String {
    format!("order_id: {order_id}, activity_id: {activity_id}")
}

#[derive(Deserialize)]
struct PersonalInfo {
    first_name: String,
    second_name: String,
    age: usize,
    city: String,
}

async fn personal_data(QueryParams(qp): QueryParams<PersonalInfo>) -> Html {
    let h = format!(
        "<!DOCTYPE html>
        <html lang='en'>
          <head>
            <meta charset='utf-8'>
            <title>Hello!</title>
          </head>
          <body>
            <h1>Hello! {}</h1>
            <p>First Name: {}</p>
            <p>Second Name: {}</p>
            <p>Age: {}</p>
            <p>City: {}</p>
          </body>
        </html>
    ",
        qp.first_name, qp.first_name, qp.second_name, qp.age, qp.city
    );
    Html(h)
}

async fn wildcard_handler(WildCardParam(w): WildCardParam<String>) -> String {
    format!("remainder of path is: {}", w)
}

#[async_std::main]
async fn main() {
    let mut app = Router::new();
    app.bind_address("127.0.0.1:8080").await.unwrap();

    // register handlers
    app.register_path(Method::Get, "/", handle_get_base);
    app.register_path(Method::Post, "/", handle_post_base);
    app.register_path(Method::Get, "/order/:order_id", handle_get_path_params);
    app.register_path(
        Method::Get,
        "/order/:order_id/activity/:activity_id",
        handle_get_path_params_with_two,
    );
    app.register_path(Method::Get, "/path", handle_get_query_params);
    app.register_path(Method::Get, "/empty", handle_post_empty_reply);
    app.register_path(Method::Get, "/personal-info", personal_data);
    app.register_path(Method::Get, "/test/*", wildcard_handler);

    app.listen().await.unwrap();
}
