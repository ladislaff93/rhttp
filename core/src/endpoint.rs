use std::{marker::PhantomData, ops::{Deref, DerefMut}};
use http::request::Request;

use crate::{handler::Handler, response::Response};

pub struct Endpoint {
    pub path: &'static str,
    pub handler: BoxedHandler,
    pub dynamic_path: bool
}

impl Endpoint {
    pub fn new<H, T>(path: &'static str, handler: H) -> Self 
    where
        H: Handler<T> + 'static,
        T: 'static,
    {
        let mut dynamic_path = false; 
        if path.contains("{") && path.contains("}") {
           dynamic_path = true; 
        }
        Self {path, handler: BoxedHandler::from_handler(handler), dynamic_path} 
    }
}

pub struct BoxedHandler(Box<dyn ErasedIntoHandler>);

impl BoxedHandler {
    pub fn from_handler<H, T>(handler: H) -> Self
    where
        H: Handler<T> + 'static,
        T: 'static,
    {
        Self(Box::new(HandlerWrapper {
            handler,
            marker:PhantomData
        }))
    }
}

pub trait ErasedIntoHandler {
    fn call(&self, request:&Request) -> Response;
}

pub struct HandlerWrapper<H, T>
where
    H: Handler<T>,
    T: 'static 
{
    pub(crate) handler: H,
    marker: std::marker::PhantomData<T>
}

impl<H, T> ErasedIntoHandler for HandlerWrapper<H, T>
where
    H: Handler<T>,
{
    fn call(&self, request: &Request) -> Response {
        self.handler.call(request)
    }
}

impl Deref for BoxedHandler {
    type Target = Box<dyn ErasedIntoHandler>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxedHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}