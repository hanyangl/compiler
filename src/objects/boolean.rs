use super::{Object, ObjectType, Hashable, HashKey, Objects, null::Null};

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
  pub value: bool,
}

impl Object for Boolean {
  fn object_type(&self) -> ObjectType {
    ObjectType::BOOLEAN
  }

  fn string(self) -> String {
    self.value.to_string()
  }
}

impl Hashable for Boolean {
  fn hashkey(self) -> HashKey {
    let value: u64;

    if self.value == true {
      value = 1;
    } else {
      value = 0;
    }

    HashKey {
      object_type: self.object_type(),
      value,
    }
  }
}

impl Boolean {
  pub fn new(value: bool) -> Box<Objects> {
    Box::new(Objects::BOOLEAN(Boolean { value }))
  }
}

pub fn is_truthy(object: Box<Objects>) -> bool {
  // When is a null object or false boolean object
  if object == Null::new() || object == Boolean::new(false) {
    return false;
  }

  // Default
  return true;
}
