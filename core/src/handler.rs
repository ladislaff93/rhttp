use http::request::Request;

use crate::{from_request::FromRequest, response::{IntoResponse, Response}};

pub trait Handler<T> {
    fn call(&self, request: &Request) -> Response;
}

impl <F, R> Handler<((),)> for F 
where
    F: Fn() -> R,
    R: IntoResponse
{
    fn call(&self, _: &Request) -> Response {
        let res = self();
        res.into_response()
    }
}

impl <F, T1, R> Handler<(T1,)> for F 
where 
    F: Fn(T1) -> R,
    T1: FromRequest,
    R: IntoResponse
{
    fn call(&self, request: &Request) -> Response {
        let args = T1::extract(&request);
        let resp = self(args);
        resp.into_response()
    }
}

impl <F, T1, T2, R> Handler<(T1, T2,)> for F 
where 
    F: Fn(T1, T2) -> R,
    T1: FromRequest,
    T2: FromRequest,
    R: IntoResponse
{
    fn call(&self, request: &Request) -> Response {
        let args_1 = T1::extract(&request);
        let args_2 = T2::extract(&request);
        let resp = self(args_1, args_2);
        resp.into_response()
    }
}
