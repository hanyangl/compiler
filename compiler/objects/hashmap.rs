use super::{Objects, Object};

#[derive(Debug, Clone, PartialEq)]
pub struct HashItem {
  pub key: Box<Objects>,
  pub value: Box<Objects>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashMap {
  pub data: Vec<HashItem>,
}

impl Object for HashMap {
  fn string(self) -> String {
    let mut data: Vec<String> = Vec::new();

    for item in self.data {
      data.push(format!(
        "{}: {}",
        item.key.string(),
        item.value.string(),
      ));
    }

    format!("{{ {} }}", data.join(", "))
  }
}