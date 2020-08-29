use super::{Object, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct Print {
  pub value: String,
}

impl Object for Print {
  fn string(self) -> String {
    self.value
  }
}

impl Print {
  pub fn new(value: String) -> Box<Objects> {
    Box::new(Objects::PRINT(Print { value }))
  }
}
