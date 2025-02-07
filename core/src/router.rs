use crate::{
    endpoint::{BoxedHandler, Endpoint},
    handler::Handler,
    incoming::Incoming,
    radix_tree::RadixTree,
    EndpointId,
};

use async_std::{
    io::{BufReader, Read, Write},
    net::TcpListener,
};
use futures::AsyncBufReadExt;
use futures::AsyncWriteExt;
use futures::StreamExt;
use http::common::RhttpError;
use http::common::RhttpError::{
    HandlerNotFound, ListenerDefined, ListenerNotDefined, ParsingRequestErr,
};
use http::{
    method::Method,
    response::{IntoResponse, Response},
    status_code::Status,
};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::Arc;

pub struct Router {
    hasher: DefaultHasher,
    routes: HashMap<Method, RadixTree>,
    handlers: HashMap<u64, Arc<Endpoint>>,
    listener: Option<TcpListener>,
}

impl Default for Router {
    fn default() -> Self {
        Router::new()
    }
}

impl Router {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        for method in Method::iterator() {
            routes.insert(*method, RadixTree::new());
        }
        Self {
            hasher: DefaultHasher::new(),
            routes,
            handlers: HashMap::new(),
            listener: None,
        }
    }

    fn hash_endpoint(&mut self, method: Method, path: &'static str) -> u64 {
        EndpointId { method, path }.hash(&mut self.hasher);
        self.hasher.finish()
    }

    pub fn register_path<H, T>(&mut self, method: Method, path: &'static str, handler: H)
    where
        H: Handler<T> + Send + Sync + 'static,
        T: Send + Sync + 'static,
    {
        let new_endpoint = Arc::new(Endpoint::new(handler));
        let endpoint_id = self.hash_endpoint(method, path);
        self.handlers.insert(endpoint_id, new_endpoint);
        self.routes
            .get_mut(&method)
            .expect("Method are already pre-populated!")
            .insert(path, endpoint_id);
    }

    pub async fn bind_address(&mut self, address: &str) -> Result<(), RhttpError> {
        if self.listener.is_none() {
            self.listener = Some(TcpListener::bind(address).await?);
            Ok(())
        } else {
            Err(ListenerDefined)
        }
    }

    async fn handle_request<S: Read + Write + Unpin>(
        &self,
        stream: &mut S,
    ) -> Result<Response, RhttpError> {
        let request_parts = Self::load_request(stream).await?;
        let mut request = Incoming::from(request_parts)?;
        let handler = self.get_handler(&mut request)?;
        handler.call(request).await
    }

    pub async fn listen(&self) -> Result<(), RhttpError> {
        let listener = self.listener.as_ref().ok_or(ListenerNotDefined)?;

        while let Some(Ok(mut stream)) = listener.incoming().next().await {
            let response = match self.handle_request(&mut stream).await {
                Ok(r) => r,
                Err(err) => match err {
                    HandlerNotFound(_) => Status::BadRequest.into_response(),
                    _ => Status::InternalServerError.into_response(),
                },
            };
            let ser = response.serialize();
            stream.write_all(&ser).await?;
        }
        Ok(())
    }

    pub async fn load_request<S: Read + Write + Unpin>(
        stream: &mut S,
    ) -> Result<String, RhttpError> {
        let mut buf_reader = BufReader::new(stream);
        if let Ok(load_buffer) = buf_reader.fill_buf().await {
            let load_buffer = load_buffer.to_vec();
            buf_reader.consume_unpin(load_buffer.len());
            Ok(String::from_utf8(load_buffer)?)
        } else {
            Err(ParsingRequestErr(String::new()))
        }
    }

    pub fn get_handler(&self, incoming: &mut Incoming) -> Result<&BoxedHandler, RhttpError> {
        let method = incoming.get_request_method();
        let path = incoming.get_request_path();
        let tree = self.routes.get(method).expect("Map of Methods!");

        let (endpoint_id, params) = tree.find(path).ok_or(HandlerNotFound(format!(
            "No handler found for path {} and method {}",
            path,
            method.to_str()
        )))?;
        incoming.set_path_params(params);
        Ok(self
            .handlers
            .get(&endpoint_id)
            .expect("For provided endpoint id should be registered handler"))
    }
}

#[cfg(test)]
mod tests {
    // use std::collections::VecDeque;
    // use crate::from_request::PathParam;
    //
    use super::*;
    //
    fn setup_router() -> Router {
        Router::new()
    }

    #[async_std::test]
    async fn test_double_bind_listener() {
        let mut router = setup_router();
        assert!(router.listener.is_none());
        router.bind_address("127.0.0.1:8080").await;
        assert!(router.listener.is_some());
        router.bind_address("127.0.0.1:8080").await;
    }

    // #[test]
    // fn test_get_dynamic_path_register_one_find_one() {
    //     let mut router = setup_router();
    //     let endpoint = Arc::new(Endpoint::new("/{r}", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /{r}"})));
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET /hello HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, true);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()).unwrap(), "/hello".to_string());
    // }
    //
    // #[test]
    // fn test_get_dynamic_path_register_not_dynamic_find_none() {
    //     let mut router = setup_router();
    //     let endpoint = Arc::new(Endpoint::new("/", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /"})));
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET / HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, false);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()), None);
    // }
    //
    // #[test]
    // fn test_get_dynamic_path_register_dynamic_multiple_arguments_find_one() {
    //     let mut router = setup_router();
    //     let endpoint = Arc::new(Endpoint::new("/{a}/{b}/{c}/{d}/{e}/{f}", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /{a}/{b}/{c}/{d}/{e}/{f}"})));
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET /1/2/3/4/5/6 HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, true);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()).unwrap(), "/1/2/3/4/5/6");
    // }
    //
    // #[test]
    // fn test_get_handler() {
    //     let mut router = setup_router();
    //     let endpoint = Arc::new(Endpoint::new("/", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /"})));
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET / HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     let handler = router.get_handler(&mut request);
    //     assert_eq!(handler.unwrap().call(&request), Response{body: "Register Get Method on /".to_string()});
    // }
    //
    // #[test]
    // fn test_get_handler_multiple_endoints_call_correct() {
    //     let mut router = setup_router();
    //     let endpoint = Arc::new(
    //         Endpoint::new("/{r}", BoxedIntoRoute::from_handler(|PathParam(r): PathParam<String>| -> String {format!("Register Get Method on /{r}")}))
    //     );
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET /hello HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     let handler = router.get_handler(&mut request);
    //     assert_eq!(handler.unwrap().call(&request), Response{body: "Register Get Method on /hello".to_string()});
    // }
    //
    // #[test]
    // fn test_register_path() {
    //     let mut router = setup_router();
    //
    //     router.register_path(
    //         Method::Get,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Get Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Post,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Post Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Delete,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Delete Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Put,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Put Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Options,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Options Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Head,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Head Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Trace,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Trace Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Connect,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Connect Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Patch,
    //         Arc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Patch Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     assert_eq!(router.routes.len(), 9);
    //     for route in router.routes {
    //         assert_eq!(route.1.len(), 1)
    //     }
    //
    // }
}
