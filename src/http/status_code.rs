use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Copy, Clone, Debug)]
pub enum StatusCode {
  BadRequest = 400,
  NotFound = 404,
  Ok = 200,
}

impl StatusCode {
  pub fn reason_phrase(&self) -> &str {
    match self {
      Self::BadRequest => "Bad Request",
      Self::NotFound => "Not Found",
      Self::Ok => "OK",
    }
  }
}

impl Display for StatusCode {
  fn fmt(&self, f: &mut Formatter) -> FormatResult {
    write!(f, "{}", *self as u16)
  }
}
