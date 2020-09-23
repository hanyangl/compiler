use super::{
  Null,
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
  elements: Vec<Box<Objects>>,
}

impl Object for Array {
  fn string(&self) -> String {
    let mut elements: Vec<String> = Vec::new();

    for element in self.elements.iter() {
      elements.push(element.string());
    }

    format!("[{}]", elements.join(", "))
  }
}

impl Array {
  pub fn new(elements: Vec<Box<Objects>>) -> Box<Objects> {
    Box::new(Objects::ARRAY(Array { elements }))
  }

  pub fn get_elements(&self) -> Vec<Box<Objects>> {
    self.elements.clone()
  }

  pub fn add_element(&mut self, object: &Box<Objects>) {
    self.elements.push(object.clone());
  }

  pub fn replace_element(&mut self, index: usize, object: &Box<Objects>) {
    if index >= self.elements.len() {
      while self.elements.len() < index {
        self.add_element(&Null::new());
      }

      self.add_element(object);
    } else {
      self.elements[index] = object.clone();
    }
  }
}
