use super::{Object, ObjectType, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
  pub value: Box<Objects>,
}

impl Object for Return {
  fn object_type(&self) -> ObjectType {
    ObjectType::RETURN
  }

  fn string(self) -> String {
    self.value.string()
  }
}

impl Return {
  pub fn new(value: Box<Objects>) -> Box<Objects> {
    Box::new(Objects::RETURN(Return { value }))
  }
}
