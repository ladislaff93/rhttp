use crate::{handler::Handler, incoming::Incoming};
use http::{common::RhttpError, response::Response};
use std::{
    fmt::Debug, future::Future, marker::PhantomData, ops::{Deref, DerefMut}, pin::Pin
};

pub(crate) type BoxedHandler = Box<dyn ErasedIntoHandler + Send + Sync>;

pub(crate) type PinnedBoxedResponse<'r> =
    Pin<Box<dyn Future<Output = Result<Response<'r>, RhttpError>> + 'r + Send + Sync>>;

pub(crate) struct Endpoint(BoxedHandler);

impl Endpoint {
    pub(crate) fn new<H, T>(handler: H) -> Self
    where
        H: Handler<T> + Send + Sync + 'static,
        T: Send + Sync + 'static,
    {
        Self(Box::new(HandlerWrapper::new(handler)))
    }
}

impl Debug for Endpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Endpoint").finish()
    }
}

impl Deref for Endpoint {
    type Target = BoxedHandler;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Endpoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait ErasedIntoHandler {
    fn call(&self, request: Incoming) -> PinnedBoxedResponse;
}

#[derive(Debug)]
pub struct HandlerWrapper<H, T>
where
    H: Handler<T> + Send + Sync,
    T: 'static,
{
    pub(crate) handler: H,
    marker: std::marker::PhantomData<T>,
}

impl<H, T> HandlerWrapper<H, T>
where
    H: Handler<T> + Send + Sync,
    T: 'static,
{
    fn new(handler: H) -> Self {
        HandlerWrapper {
            handler,
            marker: PhantomData,
        }
    }
}

impl<H, T> ErasedIntoHandler for HandlerWrapper<H, T>
where
    H: Handler<T> + Send + Sync,
{
    fn call(&self, request: Incoming) -> PinnedBoxedResponse {
        Box::pin(self.handler.call(request))
    }
}
