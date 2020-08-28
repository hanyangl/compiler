use super::{Object, Hashable, HashKey, Objects};

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
    Box::new(Objects::BOOLEAN(Boolean { value }))
  }
}
