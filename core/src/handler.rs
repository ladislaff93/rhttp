use std::future::Future;
use http::response::{IntoResponse, Response};
use crate::{from_request::FromRequest, incoming::Incoming};


pub trait Handler<T> {
    fn call(&self, incoming: &Incoming) -> impl Future<Output=Response>;
}

impl <F, Fut, R> Handler<((),)> for F 
where
    F: Fn() -> Fut,
    Fut: Future<Output=R>,
    R: IntoResponse
{
    fn call(&self, _: &Incoming) -> impl Future<Output=Response> {
        async move {
            let res = self().await;
            res.into_response()
        }
    }
}

impl <F, Fut, T1, R> Handler<(T1,)> for F 
where 
    F: Fn(T1) -> Fut,
    T1: FromRequest,
    Fut: Future<Output=R>,
    R: IntoResponse
{
    fn call(&self, incoming: &Incoming) -> impl Future<Output=Response> {
        async move {
            let args = T1::extract(incoming);
            let resp = self(args).await;
            resp.into_response()
        }
    }
}

impl <F, Fut, T1, T2, R> Handler<(T1, T2,)> for F 
where 
    F: Fn(T1, T2) -> Fut,
    T1: FromRequest,
    T2: FromRequest,
    Fut: Future<Output=R>,
    R: IntoResponse
{
    fn call(&self, incoming: &Incoming) -> impl Future<Output=Response> {
        async move {
            let args_1 = T1::extract(incoming);
            let args_2 = T2::extract(incoming);
            let resp = self(args_1, args_2).await;
            resp.into_response()
        }
    }
}
