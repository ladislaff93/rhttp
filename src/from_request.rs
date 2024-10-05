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

impl <T> FromRequest for QueryParams<T>
where 
    T: for<'a>Deserialize<'a>
{
    fn extract(req: &HttpRequest) -> Self {
        Self::from_path(req.query_params.clone())
    }
}


pub struct QueryParams<T>(pub T);

impl <T> QueryParams<T> 
where 
    T: for<'a>Deserialize<'a>
{
    fn from_path(q: String) -> Self {
        Self(serde_qs::from_str::<T>(&q).unwrap())
    }
}

