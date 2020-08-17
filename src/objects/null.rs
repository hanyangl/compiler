use super::{Object, ObjectType, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct Null;

impl Object for Null {
  fn object_type(&self) -> ObjectType {
    ObjectType::NULL
  }

  fn string(self) -> String {
    String::from("null")
  }
}

impl Null {
  pub fn new() -> Box<Objects> {
    Box::new(Objects::NULL(Null {}))
  }
}
