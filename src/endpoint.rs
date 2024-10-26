use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use crate::{handler::Handler, http_request::HttpRequest, response::Response};

pub struct Endpoint {
    pub path: &'static str,
    pub handler: BoxedIntoRoute,
    pub dynamic_path: bool
}

impl Endpoint {
    pub fn new(path: &'static str, handler: BoxedIntoRoute) -> Self {
        let mut dynamic_path = false; 
        if path.contains("{") && path.contains("}") {
           dynamic_path = true; 
        }
        Self {path, handler, dynamic_path} 
    }
}

pub struct BoxedIntoRoute(Box<dyn ErasedIntoRoute>);

impl BoxedIntoRoute {
    pub fn from_handler<H, T>(handler: H) -> Self
    where
        H: Handler<T> + 'static,
        T: 'static,
    {
        Self(Box::new(MakeErasedHandler {
            handler,
            marker:PhantomData
        }))
    }
}

pub trait ErasedIntoRoute {
    fn call(&self, request:&HttpRequest) -> Response;
}

pub struct MakeErasedHandler<H, T>
where
    H: Handler<T>,
    T: 'static 
{
    pub(crate) handler: H,
    marker: std::marker::PhantomData<T>
}

impl<H, T> ErasedIntoRoute for MakeErasedHandler<H, T>
where
    H: Handler<T>,
{
    fn call(&self, request:&HttpRequest) -> Response {
        self.handler.call(request)
    }
}

impl Deref for BoxedIntoRoute {
    type Target = Box<dyn ErasedIntoRoute>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxedIntoRoute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
