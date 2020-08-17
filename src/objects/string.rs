use super::{Object, ObjectType, Hashable, HashKey, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct StringO {
  pub value: String,
}

impl Object for StringO {
  fn object_type(&self) -> ObjectType {
    ObjectType::STRING
  }

  fn string(self) -> String {
    self.value
  }
}

impl Hashable for StringO {
  fn hashkey(self) -> HashKey {
    let mut value: u64 = 0;

    for byte in self.value.as_bytes() {
      value += byte.to_string().parse::<u64>().unwrap();
    }

    HashKey {
      object_type: self.object_type(),
      value,
    }
  }
}

impl StringO {
  pub fn new(value: String) -> Box<Objects> {
    Box::new(Objects::STRING(StringO { value }))
  }
}
