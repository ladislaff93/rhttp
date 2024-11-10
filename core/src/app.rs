use std::io::Write;
use std::net::TcpListener;
use std::rc::Rc;
use http::method::Method;

use crate::endpoint::Endpoint;
use crate::handler::Handler;
use crate::router::Router;

pub struct App {
    pub router: Router,
    pub listener: Option<TcpListener>
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            listener: None
        }
    }

    pub fn bind_address(&mut self, address: &str) {
        if self.listener.is_none() {
            self.listener = Some(TcpListener::bind(address)
                .expect("Should bind to empty port! Is port empty?"));
        }
    }

    pub fn listen(&self) {
        for stream_res in self.listener.as_ref().unwrap().incoming() {
            if let Ok(mut stream) = stream_res {
                let res = self.router.handle_request(&stream);
                stream.write_all(res.body.as_bytes()).unwrap();
            }
        }
    }

    pub fn register_path<H, T>(&mut self, method: Method, path: &'static str, handler: H) 
    where 
        H: Handler<T> + 'static,
        T: 'static
    {
        let endpoint = Rc::new(
            Endpoint::new(
                path,
                handler
            )
        );
        self.router.register_path(method, endpoint);
    }
}
