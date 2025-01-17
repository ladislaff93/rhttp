use crate::{from_request::FromRequest, incoming::Incoming};
use http::{
    common::RhttpError,
    response::{IntoResponse, Response},
};
use std::future::Future;

pub trait Handler<T> {
    fn call(
        &self,
        incoming: Incoming,
    ) -> impl Future<Output = Result<Response, RhttpError>> + Send + Sync;
}

impl<F, Fut, R> Handler<((),)> for F
where
    F: Fn() -> Fut + Send + Sync,
    Fut: Future<Output = R> + Send + Sync,
    R: IntoResponse,
{
    fn call(
        &self,
        _: Incoming,
    ) -> impl Future<Output = Result<Response, RhttpError>> + Send + Sync {
        async move {
            let res = self().await;
            Ok(res.into_response())
        }
    }
}

impl<F, Fut, T1, R> Handler<(T1,)> for F
where
    F: Fn(T1) -> Fut + Send + Sync,
    T1: FromRequest + Send + Sync,
    Fut: Future<Output = R> + Send + Sync,
    R: IntoResponse,
{
    fn call(
        &self,
        incoming: Incoming,
    ) -> impl Future<Output = Result<Response, RhttpError>> + Send + Sync {
        async move {
            let args = T1::extract(&incoming)?;
            let resp = self(args).await;
            Ok(resp.into_response())
        }
    }
}

impl<F, Fut, T1, T2, R> Handler<(T1, T2)> for F
where
    F: Fn(T1, T2) -> Fut + Send + Sync,
    T1: FromRequest + Send + Sync,
    T2: FromRequest + Send + Sync,
    Fut: Future<Output = R> + Send + Sync,
    R: IntoResponse,
{
    fn call(
        &self,
        incoming: Incoming,
    ) -> impl Future<Output = Result<Response, RhttpError>> + Send + Sync {
        async move {
            let args_1 = T1::extract(&incoming)?;
            let args_2 = T2::extract(&incoming)?;
            let resp = self(args_1, args_2).await;
            Ok(resp.into_response())
        }
    }
}
