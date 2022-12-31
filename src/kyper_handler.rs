use crate::server::Handler;
use crate::http::{Response, Request, Method, StatusCode};

pub struct KyperHandler {

}

impl Handler for KyperHandler {
    fn handle_request(&mut self, request: Request) -> Response {
        match request.method() {
            Method::GET => {
                Response::new(StatusCode::Ok, Some("s".to_string()))
            },
            _ => Response::new(StatusCode::BadRequest, None)
        }
    }

    fn handle_bad_request (&mut self, e: crate::http::request::ParseError) -> Response {
        println!("An Error occured:: {}", e);
        Response::new(crate::http::StatusCode::BadRequest, None)
    }
}