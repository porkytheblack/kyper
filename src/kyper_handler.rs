use std::fs;

use crate::server::Handler;
use crate::http::{Response, Request, Method, StatusCode};

pub struct KyperHandler {
    public_path: String
}

impl KyperHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    pub fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        if !path.contains("..") {
            fs::read_to_string(path).ok()
        } else {
            println!("Directory traversal attempted");
            None
        }
    }
}

impl Handler for KyperHandler {
    fn handle_request(&mut self, request: Request) -> Response {
        match request.method() {
            Method::GET => {
                match request.path() {
                    "/" => match self.read_file("build/index.html") {
                        Some(data) => {
                            Response::new(StatusCode::Ok, Some(data))
                        },
                        None =>  Response::new(StatusCode::BadRequest, None)
                    },
                    path => {
                        let p = format!("build/{}", path);
                        match self.read_file(&p[..]) {
                            Some(data) => {
                                Response::new(StatusCode::Ok, Some(data))
                            },
                            None =>  Response::new(StatusCode::BadRequest, None)
                        }
                    }
                }
                
            },
            _ => Response::new(StatusCode::BadRequest, None)
        }
    }

    fn handle_bad_request (&mut self, e: crate::http::request::ParseError) -> Response {
        println!("An Error occured:: {}", e);
        Response::new(crate::http::StatusCode::BadRequest, None)
    }
}