use super::{
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ForIn {
  name: String,
  elements: Vec<Box<Objects>>,
}

impl Object for ForIn {
  fn string(&self) -> String {
    let mut elements: Vec<String> = Vec::new();

    for element in self.elements.iter() {
      elements.push(element.string());
    }

    format!("[{}]", elements.join(", "))
  }
}

impl ForIn {
  pub fn new(name: String, elements: Vec<Box<Objects>>) -> Box<Objects> {
    Box::new(Objects::FORIN(Self { name, elements }))
  }

  pub fn get_name(&self) -> String {
    self.name.clone()
  }

  pub fn get_elements(&self) -> Vec<Box<Objects>> {
    self.elements.clone()
  }
}
