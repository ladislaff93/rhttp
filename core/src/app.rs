use async_std::io::WriteExt;
use async_std::net::TcpListener;
use async_std::stream::StreamExt;
use std::rc::Rc;
use http::method::Method;

use crate::endpoint::Endpoint;
use crate::handler::Handler;
use crate::incoming::Incoming;
use crate::outcoming::{Outcoming, Serialize};
use crate::router::Router;

pub struct App {
    pub router: Router,
    pub listener: Option<TcpListener>
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            listener: None
        }
    }

    pub async fn bind_address(&mut self, address: &str) {
        if self.listener.is_none() {
            self.listener = Some(TcpListener::bind(address)
                .await
                .expect("Should bind to empty port! Is port empty?"));
        }
    }

    pub async fn listen(&self) {
        while let Some(Ok(mut stream)) = self.listener.as_ref().unwrap().incoming().next().await {
            let request_parts = Router::load_request(&mut stream).await.expect("Valid utf8 coded message from client");
            let mut request = Incoming::from(&request_parts).unwrap();
            println!("REQUEST: {:#?}", request);
            let res = if let Some(handler) = self.router.get_handler(&mut request) {
                 handler.call(&request).await
            } else {
                Router::handler_not_found()
            };
            println!("RESPONSE: {:?}", res);
            let out = Outcoming::new(res);
            let ser = out.serialize();
            println!("SERIALIZE MSG: {:?}", String::from_utf8(ser.clone()));
            stream.write_all(&ser).await.unwrap();
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
