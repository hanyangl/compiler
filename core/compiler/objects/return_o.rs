use super::{
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnO {
  value: Box<Objects>,
}

impl Object for ReturnO {
  fn string(&self) -> String {
    self.value.clone().string()
  }
}

impl ReturnO {
  pub fn new(value: Box<Objects>) -> Box<Objects> {
    Box::new(Objects::RETURN(ReturnO { value }))
  }

  pub fn get_value(&self) -> Box<Objects> {
    self.value.clone()
  }
}
