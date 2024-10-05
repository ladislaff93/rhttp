use crate::{from_request::FromRequest, http_request::HttpRequest, response::Response};

pub trait Handler<T> {
    fn call(&self, request: &HttpRequest) -> Response;
}

impl <F> Handler<((),)> for F 
where
    F: Fn() -> String 
{
    fn call(&self, _: &HttpRequest) -> Response {
        let res = self();
        Response {
            body: res
        }
    }
}

impl <F, T1> Handler<(T1,)> for F 
where 
    F: Fn(T1) -> String,
    T1: FromRequest,
{
    fn call(&self, request: &HttpRequest) -> Response {
        let args = T1::extract(&request);
        let resp = self(args);
        Response {
            body: resp
        }
    }
}
