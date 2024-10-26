use std::{collections::HashMap, net::TcpStream, rc::Rc};
use crate::{common::FINAL_CRLF, endpoint::{BoxedIntoRoute, Endpoint}, http_request::{HttpMethod, HttpRequest}, response::Response};

pub struct Router {
    routes: HashMap<HttpMethod, Vec<Rc<Endpoint>>>
}

impl Router {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        for method in HttpMethod::iterator() {
            routes.insert(*method, Vec::<Rc<Endpoint>>::new());
        }
        Self {routes}
    }

    pub fn register_path(&mut self, method: HttpMethod, endpoint: Rc<Endpoint>) {
        self.routes.entry(method).and_modify(|v| v.push(endpoint));
    }

    /*
        we need to resolve incoming request with path params /foo/5/bar/10 
        into handler that is registered under                /foo/{usize}/bar/{usize}
        we can split both incoming and registered path into vec 
        then we find index of the dynamic parts of the path
        and then we try to call the handler and try to cast into expected types
        if it fails we look for another handler and path if we reach end we return 404 not found / 400 bad request
        
        to avoid many urls in one vec we can split them into dyn and static during path registration 
        HttpMethod, HashMap<RouteType, Vec<Endpoints>>
    */
    pub fn handle_request(&self, request: &TcpStream) -> Response {
        let mut request = HttpRequest::new(request);
        println!("Req: {:#?}", request);
        if let Some(handler) = self.get_handler(&mut request) {
            return handler.call(&request);
        }
        return Self::handler_not_found();
    }

    fn get_handler(&self, request: &mut HttpRequest) -> Option<&BoxedIntoRoute> {
        for endpoint in self.routes.get(&request.method).expect("Map of Http Methods!") {
            if !endpoint.dynamic_path {
                if endpoint.path == request.path {
                    return Some(&endpoint.handler);
                }
            }

            if self.match_dynamic_path(request, endpoint.clone()) {
                return Some(&endpoint.handler);
            }
        } 
        return None;
    }

    fn handler_not_found() -> Response {
        Response {
            body: format!("HTTP/1.1 500 Internal Server Error{FINAL_CRLF}")
        }
    }

    fn match_dynamic_path<'r>(&self, request: &mut HttpRequest<'r>, endpoint: Rc<Endpoint>) -> bool {
        let register_path_splitted = endpoint.path.split("/").filter(|s|!s.is_empty()).collect::<Vec<&str>>();
        let incoming_path_splitted = request.path.split("/").filter(|s|!s.is_empty()).collect::<Vec<&str>>();

        let mut path_params: Vec<&'r str> = register_path_splitted.iter()
            .enumerate()
            .filter_map(|(idx, &sub)| {
                if sub.starts_with("{") && sub.ends_with("}") {
                    incoming_path_splitted.get(idx).map(|&s| s)
                } else {
                    None
                }
            }).collect();

        if path_params.len() == 0 {
            return false;
        }

        request.add_path_params(path_params.clone());

        register_path_splitted.iter()
            .map(|&sub| {
                if sub.starts_with("{") && sub.ends_with("}") {
                    "/".to_owned() + path_params.remove(0)
                } else {
                    "/".to_owned() + sub
                }}
            )
            .collect::<String>() == request.path
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
    //     router.register_path(HttpMethod::Get, endpoint.clone());
    //     let mut request = HttpRequest::new(String::from("GET /hello HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, true);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()).unwrap(), "/hello".to_string());
    // }
    //
    // #[test]
    // fn test_get_dynamic_path_register_not_dynamic_find_none() {
    //     let mut router = setup_router();
    //     let endpoint = Rc::new(Endpoint::new("/", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /"})));
    //     router.register_path(HttpMethod::Get, endpoint.clone());
    //     let mut request = HttpRequest::new(String::from("GET / HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, false);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()), None);
    // }
    //
    // #[test]
    // fn test_get_dynamic_path_register_dynamic_multiple_arguments_find_one() {
    //     let mut router = setup_router();
    //     let endpoint = Rc::new(Endpoint::new("/{a}/{b}/{c}/{d}/{e}/{f}", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /{a}/{b}/{c}/{d}/{e}/{f}"})));
    //     router.register_path(HttpMethod::Get, endpoint.clone());
    //     let mut request = HttpRequest::new(String::from("GET /1/2/3/4/5/6 HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     assert_eq!(endpoint.dynamic_path, true);
    //     assert_eq!(router.get_dynamic_path(&mut request, endpoint.clone()).unwrap(), "/1/2/3/4/5/6");
    // }
    //
    // #[test]
    // fn test_get_handler() {
    //     let mut router = setup_router();
    //     let endpoint = Rc::new(Endpoint::new("/", BoxedIntoRoute::from_handler(|| -> &str {"Register Get Method on /"})));
    //     router.register_path(HttpMethod::Get, endpoint.clone());
    //     let mut request = HttpRequest::new(String::from("GET / HTTP/1.1").bytes().collect::<VecDeque<u8>>());
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
    //     router.register_path(HttpMethod::Get, endpoint.clone());
    //     let mut request = HttpRequest::new(String::from("GET /hello HTTP/1.1").bytes().collect::<VecDeque<u8>>());
    //     let handler = router.get_handler(&mut request);
    //     assert_eq!(handler.unwrap().call(&request), Response{body: "Register Get Method on /hello".to_string()});
    // }
    //
    // #[test]
    // fn test_register_path() {
    //     let mut router = setup_router();
    //
    //     router.register_path(
    //         HttpMethod::Get,
    //         Rc::new(Endpoint::new("/", 
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Get Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         HttpMethod::Post,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Post Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         HttpMethod::Delete,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Delete Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         HttpMethod::Put,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Put Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         HttpMethod::Options,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Options Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         HttpMethod::Head,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Head Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         HttpMethod::Trace,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Trace Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         HttpMethod::Connect,
    //         Rc::new(Endpoint::new("/",
    //             BoxedIntoRoute::from_handler(
    //                 || -> String {
    //                     "Register Connect Method on /".to_string()
    //                 }
    //             )
    //         ))
    //     );
    //     router.register_path(
    //         HttpMethod::Patch,
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
