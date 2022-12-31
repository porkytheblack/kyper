use std::net::TcpListener;
use std::io::{Read};

use crate::http::request::ParseError;
use crate::http::{Request, Response, StatusCode};

pub trait Handler {
    fn handle_request (&mut self, request: Request) -> Response;

    fn handle_bad_request (&mut self, e: ParseError) -> Response {
        println!("An Error occured:: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr
        }
    }

    pub fn run(&self, handler:&mut impl Handler){
        let kyper_listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            println!("Starting Kyper Server On::{}", self.addr);

            match kyper_listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    let response = match stream.read(&mut buffer) {
                        Ok(_) => {
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    handler.handle_request(
                                        request
                                    )
                                },
                                Err(e)=>{
                                    println!("Parse Error Occured::{}", e);
                                    handler.handle_bad_request(e)
                                }
                            }
                        },
                        Err(e) => {
                            println!("An error occured {}", e);
                            handler.handle_bad_request(ParseError::InvalidEncoding)
                        }
                    };
                    if let Err(e) = response.send(&mut stream) {
                        println!("An error occured::{}", e)
                    }
                },
                Err(e)=> println!("An Error Occured:: {}", e)
            }
        }
    }
}