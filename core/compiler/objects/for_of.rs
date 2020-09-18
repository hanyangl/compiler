use super::{
  HashItem,
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ForOf {
  names: Vec<String>,
  data: Vec<HashItem>,
}

impl Object for ForOf {
  fn string(&self) -> String {
    let mut data: Vec<String> = Vec::new();

    for item in self.data.iter() {
      data.push(format!(
        "{}: {}",
        item.key,
        item.value.clone().string(),
      ));
    }

    format!(
      "{{ {} }}",
      data.join(", "),
    )
  }
}

impl ForOf {
  pub fn new(names: Vec<String>, data: Vec<HashItem>) -> Box<Objects> {
    Box::new(Objects::FOROF(Self { names, data }))
  }

  pub fn get_names(&self) -> Vec<String> {
    self.names.clone()
  }

  pub fn get_data(&self) -> Vec<HashItem> {
    self.data.clone()
  }
}
