use std::{fmt::Debug, str::FromStr};

use serde::Deserialize;

use crate::http_request::HttpRequest;

pub trait FromRequest {
    fn extract(req: &HttpRequest) -> Self;
}

impl <T1> FromRequest for (T1,)
where 
    T1: FromRequest,
{
    fn extract(req: &HttpRequest) -> Self {
        (T1::extract(req),)
    }
}

impl <T1, T2> FromRequest for (T1, T2,)
where 
    T1: FromRequest,
    T2: FromRequest,
{
    fn extract(req: &HttpRequest) -> Self {
        (T1::extract(req), T2::extract(req))
    }
}

impl <T> FromRequest for QueryParams<T>
where 
    T: for<'a>Deserialize<'a>
{
    fn extract(req: &HttpRequest) -> Self {
        Self::from_path(req.query_params.clone())
    }
}

impl <T> FromRequest for PathParam<T>
where 
    T: FromStr, 
    <T as FromStr>::Err: Debug,
    T: Debug,
    T: for<'a>Deserialize<'a>
{
    fn extract(req: &HttpRequest) -> Self {
        Self::from_path(req.path_params.clone())
    }
}


pub struct QueryParams<T>(pub T);

impl <T> QueryParams<T> 
where 
    T: for<'a>Deserialize<'a>
{
    fn from_path(q: &str) -> Self {
        Self(serde_qs::from_str::<T>(&q).unwrap())
    }
}

pub struct PathParam<T>(pub T);

impl <T> PathParam<T> 
where 
    T: FromStr, 
    T: Debug,
    T: for<'a>Deserialize<'a>,
    <T as FromStr>::Err: Debug
{
    fn from_path(mut q: Vec<&str>) -> Self {
        Self(T::from_str(q.remove(0)).unwrap())
    }
}
