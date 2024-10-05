use std::{collections::HashMap, marker::PhantomData};
use crate::{common::FINAL_CRLF, handler::Handler, http_request::{HttpMethod, HttpRequest}, response::Response};

pub struct Router {
    routes: HashMap<(String, HttpMethod), BoxedIntoRoute>
}

impl Router {
    pub fn new() -> Self {
        Self {routes: HashMap::new()}
    }

    pub fn handle_request(&self, request: HttpRequest) -> Response {
        let handler_optional = self.routes.get(&(request.path.to_string(), request.method));
        if let Some(BoxedIntoRoute(handler)) = handler_optional {
            return handler.as_ref().call(&request);
        } else {
            return Self::handler_not_found();
        }
    }

    fn handler_not_found() -> Response {
        Response {
            body: format!("HTTP/1.1 500 Internal Server Error{FINAL_CRLF}")
        }
    }

    pub fn register_path(&mut self, method: HttpMethod, path: &str, handler: BoxedIntoRoute) {
        if self.routes.contains_key(&(path.to_string(), method)) == true {
            panic!("Already registered path: {path} for given method: {method}");
        }
        self.routes.insert((path.to_string(), method), handler);
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

trait ErasedIntoRoute {
    fn call(&self, request:&HttpRequest) -> Response;
}

struct MakeErasedHandler<H, T>
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
