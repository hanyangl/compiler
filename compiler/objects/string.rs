use super::{Object, Hashable, HashKey, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct StringO {
  pub value: String,
}

impl Object for StringO {
  fn string(self) -> String {
    self.value
  }
}

impl Hashable for StringO {
  fn get_hashkey(self) -> HashKey {
    let mut value: f64 = 0.0;

    for byte in self.value.as_bytes() {
      value += byte.to_string().parse::<f64>().unwrap();
    }

    HashKey {
      value,
    }
  }
}

impl StringO {
  pub fn new(value: String) -> Box<Objects> {
    Box::new(Objects::STRING(StringO { value }))
  }
}
