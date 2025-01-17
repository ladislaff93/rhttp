use crate::{handler::Handler, incoming::Incoming};
use http::{common::RhttpError, response::Response};
use std::{
    future::Future,
    marker::PhantomData,
    ops::{Deref, DerefMut},
    pin::Pin,
};

pub(crate) struct Endpoint {
    pub handler: BoxedHandler
}

impl Endpoint {
    pub fn new<H, T>(handler: H) -> Self
    where
        H: Handler<T> + Send + Sync + 'static,
        T: 'static + Send + Sync,
    {
        Self {
            handler: BoxedHandler::from_handler(handler)
        }
    }
}

pub struct BoxedHandler(Box<dyn ErasedIntoHandler + Send + Sync>);

impl BoxedHandler {
    pub fn from_handler<H, T>(handler: H) -> Self
    where
        H: Handler<T> + Send + Sync + 'static,
        T: 'static + Send + Sync,
    {
        Self(Box::new(HandlerWrapper {
            handler,
            marker: PhantomData,
        }))
    }
}

pub trait ErasedIntoHandler {
    fn call<'r>(
        &'r self,
        request: Incoming,
    ) -> Pin<Box<dyn Future<Output = Result<Response<'r>, RhttpError>> + 'r + Send + Sync>>;
}

pub struct HandlerWrapper<H, T>
where
    H: Handler<T> + Send + Sync,
    T: 'static,
{
    pub(crate) handler: H,
    marker: std::marker::PhantomData<T>,
}

impl<H, T> ErasedIntoHandler for HandlerWrapper<H, T>
where
    H: Handler<T> + Send + Sync,
{
    fn call<'r>(
        &'r self,
        request: Incoming,
    ) -> Pin<Box<dyn Future<Output = Result<Response<'r>, RhttpError>> + 'r + Send + Sync>> {
        Box::pin(self.handler.call(request))
    }
}

impl Deref for BoxedHandler {
    type Target = Box<dyn ErasedIntoHandler + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxedHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
