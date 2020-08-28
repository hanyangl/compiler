use super::{Object, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub message: String,
}

impl Object for Error {
  fn string(self) -> String {
    format!("[ERROR] {}", self.message)
  }
}

impl Error {
  pub fn new(message: String) -> Box<Objects> {
    Box::new(Objects::ERROR(Error { message }))
  }
}
