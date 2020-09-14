use super::{
  Hashable,
  HashKey,
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
  pub value: f64,
}

impl Object for Number {
  fn string(&self) -> String {
    self.value.to_string()
  }
}

impl Hashable for Number {
  fn get_hashkey(&self) -> HashKey {
    HashKey {
      value: self.value,
    }
  }
}

impl Number {
  pub fn new(value: f64) -> Box<Objects> {
    Box::new(Objects::NUMBER(Number {
      value,
    }))
  }
}
