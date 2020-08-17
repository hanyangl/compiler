use super::{Object, ObjectType, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  pub message: String,
}

impl Object for Error {
  fn object_type(&self) -> ObjectType {
    ObjectType::ERROR
  }

  fn string(self) -> String {
    format!("Error: {}", self.message)
  }
}

impl Error {
  pub fn new(message: String) -> Box<Objects> {
    Box::new(Objects::ERROR(Error { message }))
  }
}
