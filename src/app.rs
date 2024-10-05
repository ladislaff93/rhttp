use std::io::Write;
use std::net::{TcpListener, TcpStream};
use crate::handler::Handler;
use crate::router::BoxedIntoRoute;
use crate::{http_request::{HttpMethod, HttpRequest}, router::Router};

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
                let res = self.parse_incoming_data(&stream);
                stream.write_all(res.as_bytes()).unwrap();
            }
        }
    }

    pub fn parse_incoming_data(&self, stream: &TcpStream) -> String {
        let req = HttpRequest::new(&stream);
        println!("Req: {:#?}", req);
        self.router.handle_request(req).body
    }

    pub fn register_path<H, T>(&mut self, method: HttpMethod, path: &str, handler: H) 
    where 
        H: Handler<T> + 'static,
        T: 'static
    {
        self.router.register_path(method, path, BoxedIntoRoute::from_handler(handler));
    }
}
