use crate::incoming::Incoming;
use http::common::RhttpError::{self, ParsingPathParamsErr, WildCardPathParamsErr};
use serde::Deserialize;
use std::{fmt::Debug, str::FromStr};

pub(crate) trait FromRequest {
    fn extract(req: &Incoming) -> Result<Self, RhttpError>
    where
        Self: Sized + Send + Sync;
}

impl<T1> FromRequest for (T1,)
where
    T1: FromRequest + Send + Sync,
{
    fn extract(req: &Incoming) -> Result<Self, RhttpError> {
        Ok((T1::extract(req)?,))
    }
}

impl<T1, T2> FromRequest for (T1, T2)
where
    T1: FromRequest + Send + Sync,
    T2: FromRequest + Send + Sync,
{
    fn extract(req: &Incoming) -> Result<Self, RhttpError> {
        Ok((T1::extract(req)?, T2::extract(req)?))
    }
}

impl<T> FromRequest for QueryParams<T>
where
    T: for<'a> Deserialize<'a>,
{
    fn extract(req: &Incoming) -> Result<Self, RhttpError> {
        Self::from_path(&req.query_params)
    }
}

impl<T> FromRequest for PathParam<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    T: Debug,
    T: for<'a> Deserialize<'a>,
{
    fn extract(req: &Incoming) -> Result<Self, RhttpError> {
        Self::from_path(req.path_params.clone())
    }
}

impl<T> FromRequest for WildCardParam<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
    T: Debug,
    T: for<'a> Deserialize<'a>,
{
    fn extract(req: &Incoming) -> Result<Self, RhttpError>
    where
        Self: Sized + Send + Sync,
    {
        Self::from_path(&req.wildcard_param)
    }
}

pub struct QueryParams<T>(pub T);

impl<T> QueryParams<T>
where
    T: for<'a> Deserialize<'a>,
{
    fn from_path(q: &str) -> Result<Self, RhttpError> {
        Ok(Self(serde_qs::from_str::<T>(q)?))
    }
}

pub struct PathParam<T>(pub T);

impl<T> PathParam<T>
where
    T: FromStr,
    T: Debug,
    T: for<'a> Deserialize<'a>,
    <T as FromStr>::Err: Debug,
{
    fn from_path(mut q: Vec<String>) -> Result<Self, RhttpError> {
        match T::from_str(&q.remove(0)) {
            Ok(a) => Ok(Self(a)),
            Err(_e) => Err(ParsingPathParamsErr),
        }
    }
}

pub struct WildCardParam<T>(pub T);

impl<T> WildCardParam<T>
where
    T: FromStr,
    T: Debug,
    T: for<'a> Deserialize<'a>,
    <T as FromStr>::Err: Debug,
{
    fn from_path(q: &str) -> Result<Self, RhttpError> {
        match T::from_str(q) {
            Ok(a) => Ok(Self(a)),
            Err(_e) => Err(WildCardPathParamsErr),
        }
    }
}
