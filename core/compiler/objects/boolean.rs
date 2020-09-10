use super::{
  Hashable,
  HashKey,
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
  pub value: bool,
}

impl Object for Boolean {
  fn string(self) -> String {
    self.value.to_string()
  }
}

impl Hashable for Boolean {
  fn get_hashkey(self) -> HashKey {
    HashKey {
      value: if self.value { 1.0 } else { 0.0 },
    }
  }
}

impl Boolean {
  pub fn new(value: bool) -> Box<Objects> {
    Box::new(Objects::BOOLEAN(Boolean {
      value,
    }))
  }

  pub fn is_truthy(object: Box<Objects>) -> bool {
    // When is a null object or false boolean object
    if object.clone().get_null().is_some() || object == Boolean::new(false) {
      return false;
    }

    // Default
    true
  }
}
