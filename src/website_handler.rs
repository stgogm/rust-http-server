use std::fs;

use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;

pub struct WebsiteHandler {
  public_path: String,
}

impl WebsiteHandler {
  pub fn new(public_path: String) -> Self {
    Self { public_path }
  }

  pub fn read_file(&self, file_path: &str) -> Option<String> {
    let path = format!("{}/{}", self.public_path, file_path);

    match fs::canonicalize(path) {
      Err(_) => None,
      Ok(path) => {
        if path.starts_with(&self.public_path) {
          return fs::read_to_string(path).ok();
        }

        println!("Directory Traversal Attack Attepmted: {}", &file_path);
        None
      }
    }
  }
}

impl Handler for WebsiteHandler {
  fn handle_request(&mut self, request: &Request) -> Response {
    match request.method() {
      Method::GET => match request.path() {
        "/test" => Response::new(StatusCode::Ok, self.read_file("test.html")),
        "/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
        path => match self.read_file(path) {
          Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
          None => Response::new(StatusCode::NotFound, None),
        },
      },
      _ => Response::new(StatusCode::NotFound, None),
    }
  }
}
