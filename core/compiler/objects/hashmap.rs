use super::{
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HashItem {
  pub key: String,
  pub value: Box<Objects>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashMap {
  data: Vec<HashItem>,
}

impl Object for HashMap {
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

impl HashMap {
  pub fn new(data: Vec<HashItem>) -> Box<Objects> {
    Box::new(Objects::HASHMAP(HashMap { data }))
  }

  pub fn get_data(&self) -> Vec<HashItem> {
    self.data.clone()
  }
}
