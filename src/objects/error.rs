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
    self.message
  }
}

impl Error {
  pub fn new(message: String) -> Box<Objects> {
    Box::new(Objects::ERROR(Error { message }))
  }
}

pub fn is_error(obj: Box<Objects>) -> bool {
  obj.object_type() == ObjectType::ERROR
}
