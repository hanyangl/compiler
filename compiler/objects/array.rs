use super::{Objects, Object};

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
  pub elements: Vec<Box<Objects>>,
}

impl Object for Array {
  fn string(self) -> String {
    let mut elements: Vec<String> = Vec::new();

    for element in self.elements {
      elements.push(element.string());
    }

    format!("[{}]", elements.join(", "))
  }
}

impl Array {
  pub fn new(elements: Vec<Box<Objects>>) -> Box<Objects> {
    Box::new(Objects::ARRAY(Array { elements }))
  }
}
