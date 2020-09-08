use super::{
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Null;

impl Object for Null {
  fn string(self) -> String {
    String::from("null")
  }
}

impl Null {
  pub fn new() -> Box<Objects> {
    Box::new(Objects::NULL(Null { }))
  }
}
