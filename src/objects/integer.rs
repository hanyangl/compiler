use super::{Object, ObjectType, Hashable, HashKey, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
  pub value: i64,
}

impl Object for Integer {
  fn object_type(&self) -> ObjectType {
    ObjectType::INTEGER
  }

  fn string(self) -> String {
    self.value.to_string()
  }
}

impl Hashable for Integer {
  fn hashkey(self) -> HashKey {
    HashKey {
      object_type: self.object_type(),
      value: self.string().parse().unwrap(),
    }
  }
}

impl Integer {
  pub fn new(value: i64) -> Box<Objects> {
    Box::new(Objects::INTEGER(Integer { value }))
  }
}
