use crate::http::{ParseError, Request, Response, StatusCode};

use std::convert::TryFrom;
use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, error: &ParseError) -> Response {
        println!("Failed to parse request: {}", error);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Err(e) => println!("Failed to establish a connection: {}", e),
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];

                    match stream.read(&mut buffer) {
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                        Ok(_) => {
                            println!(
                                "Received a request: \n\n{}",
                                String::from_utf8_lossy(&buffer)
                            );

                            let response = match Request::try_from(&buffer[..]) {
                                Err(error) => handler.handle_bad_request(&error),
                                Ok(request) => handler.handle_request(&request),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                    }
                }
            }
        }
    }
}
