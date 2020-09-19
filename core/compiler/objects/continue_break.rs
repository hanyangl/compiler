use super::{
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Continue {}

impl Object for Continue {
  fn string(&self) -> String {
    String::from("continue")
  }
}

impl Continue {
  pub fn new() -> Box<Objects> {
    Box::new(Objects::CONTINUE(Self { }))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Break {}

impl Object for Break {
  fn string(&self) -> String {
    String::from("break")
  }
}

impl Break {
  pub fn new() -> Box<Objects> {
    Box::new(Objects::BREAK(Self { }))
  }
}

