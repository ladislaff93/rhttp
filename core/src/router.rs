use futures::StreamExt;
use futures::AsyncWriteExt;
use std::{collections::BTreeMap, rc::Rc, string::FromUtf8Error};
use async_std::{io::{BufReader, Read, Write}, net::TcpListener};
use futures::AsyncBufReadExt;
use http::{method::Method, response::{IntoResponse, Response}, status_code::Status};
use crate::{endpoint::{BoxedHandler, Endpoint}, handler::Handler, incoming::Incoming, outcoming::{Outcoming, Serialize}};

pub struct Router {
    routes: BTreeMap<Method, Vec<Rc<Endpoint>>>,
    listener: Option<TcpListener>
}

impl Default for Router {
    fn default() -> Self {
        Router::new()
    }
}

impl Router {
    pub fn new() -> Self {
        let mut routes = BTreeMap::new();
        for method in Method::iterator() {
            routes.insert(*method, Vec::<Rc<Endpoint>>::new());
        }
        Self {routes, listener: None}
    }

    pub fn register_path<H, T>(&mut self, method: Method, path: &'static str, handler: H) 
    where 
        H: Handler<T> + 'static,
        T: 'static
    {
        self.routes.entry(method).and_modify(|v| v.push(Rc::new(
            Endpoint::new(
                path,
                handler
            )
        )));
    }


    // pub async fn handle_request<'r>(&self, stream: &'r mut TcpStream) -> Response {
    //     if let Ok(request_parts) = Self::load_request(stream).await {
    //        if let Ok(mut request) = Incoming::from(request_parts) {
    //             println!("REQUEST: {:#?}", request);
    //             if let Some(handler) = self.get_handler(&mut request) {
    //                 return handler.call(&request).await;
    //             }
    //         }
    //     }
    //     return Self::handler_not_found();
    // }

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
            // println!("REQUEST: {:#?}", request);
            let res = if let Some(handler) = self.get_handler(&mut request) {
                 handler.call(&request).await
            } else {
                Router::handler_not_found()
            };
            // println!("RESPONSE: {:?}", res);
            let out = Outcoming::new(res);
            let ser = out.serialize();
            // println!("SERIALIZE MSG: {:?}", String::from_utf8(ser.clone()));
            stream.write_all(&ser).await.unwrap();
        }
    }

    pub async fn load_request<S: Read + Write + Unpin>(stream: &mut S) -> Result<String, FromUtf8Error> {
        let mut buf_reader = BufReader::new(stream);
        let load_buffer = buf_reader.fill_buf().await.unwrap().to_vec();
        buf_reader.consume_unpin(load_buffer.len());
        String::from_utf8(load_buffer)
    }

    pub fn get_handler(&self, incoming: &mut Incoming) -> Option<&BoxedHandler> {
        for endpoint in self.routes.get(&incoming.get_request_method()).expect("Map of Methods!") {
            if !endpoint.dynamic_path && endpoint.path == incoming.get_request_path() {
                return Some(&endpoint.handler);
            }

            if self.match_dynamic_path(incoming, endpoint.clone()) {
                return Some(&endpoint.handler);
            }
        } 
        None
    }

    pub fn handler_not_found<'rs>() -> Response<'rs> {
        Status::InternalServerError.into_response()
    }

    fn match_dynamic_path<'r>(&self, incoming: &mut Incoming<'r>, endpoint: Rc<Endpoint>) -> bool {
        let register_path_splitted = endpoint.path.split("/").filter(|s|!s.is_empty()).collect::<Vec<&str>>();
        let incoming_path_splitted = incoming.request.request_line.path.split("/").filter(|s|!s.is_empty()).collect::<Vec<&str>>();

        let mut path_params: Vec<&'r str> = register_path_splitted.iter()
            .enumerate()
            .filter_map(|(idx, &sub)| {
                if sub.starts_with("{") && sub.ends_with("}") {
                    incoming_path_splitted.get(idx).copied()
                } else {
                    None
                }
            }).collect();

        if path_params.is_empty() {
            return false;
        }

        incoming.set_path_params(path_params.clone());

        register_path_splitted.iter()
            .map(|&sub| {
                if sub.starts_with("{") && sub.ends_with("}") {
                    "/".to_owned() + path_params.remove(0)
                } else {
                    "/".to_owned() + sub
                }}
            )
            .collect::<String>() == incoming.request.request_line.path
    }
}

#[cfg(test)]
mod tests {
    // use std::collections::VecDeque;
    // use crate::from_request::PathParam;
    //
    // use super::*;
    //
    // fn setup_router() -> Router {
    //     let new_router = Router::new();
    //     return new_router;
    // }
    //
    // #[test]
    // fn test_get_dynamic_path_register_one_find_one() {
    //     let mut router = setup_router();
    //     let endpoint = Rc::new(Endpoint::new("/{r}", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /{r}"})));
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET /hello HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, true);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()).unwrap(), "/hello".to_string());
    // }
    //
    // #[test]
    // fn test_get_dynamic_path_register_not_dynamic_find_none() {
    //     let mut router = setup_router();
    //     let endpoint = Rc::new(Endpoint::new("/", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /"})));
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET / HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, false);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()), None);
    // }
    //
    // #[test]
    // fn test_get_dynamic_path_register_dynamic_multiple_arguments_find_one() {
    //     let mut router = setup_router();
    //     let endpoint = Rc::new(Endpoint::new("/{a}/{b}/{c}/{d}/{e}/{f}", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /{a}/{b}/{c}/{d}/{e}/{f}"})));
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET /1/2/3/4/5/6 HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, true);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()).unwrap(), "/1/2/3/4/5/6");
    // }
    //
    // #[test]
    // fn test_get_handler() {
    //     let mut router = setup_router();
    //     let endpoint = Rc::new(Endpoint::new("/", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /"})));
    //     router.register_path(Method::Get, endpoint.clone());
    //     let mut request = Request::new(String::from("GET / HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     let handler = router.get_handler(&mut request);
    //     assert_eq!(handler.unwrap().call(&request), Response{body: "Register Get Method on /".to_string()});
    // }
    //
    // #[test]
    // fn test_get_handler_multiple_endoints_call_correct() {
    //     let mut router = setup_router();
    //     let endpoint = Rc::new(
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
    //         Rc::new(Endpoint::new("/", 
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Get Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Post,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Post Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Delete,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Delete Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Put,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Put Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Options,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Options Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Head,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Head Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Trace,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Trace Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Connect,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Connect Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         Method::Patch,
    //         Rc::new(Endpoint::new("/",
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
