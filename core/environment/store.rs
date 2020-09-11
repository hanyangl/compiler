use crate::compiler::Objects;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
  pub objects: HashMap<String, Box<Objects>>,
  pub outer: Option<Box<Store>>,
}

impl Store {
  pub fn new() -> Self {
    Self {
      objects: HashMap::new(),
      outer: None,
    }
  }

  pub fn from_store(outer: Store) -> Self {
    let mut store: Self = Self::new();

    store.outer = Some(Box::new(outer));

    store
  }

  pub fn get_object(&self, key: String) -> Option<Box<Objects>> {
    match self.objects.get(&key) {
      Some(object) => Some(object.clone()),
      None => match &self.outer {
        Some(outer) => outer.get_object(key),
        None => None,
      },
    }
  }

  pub fn set_object(&mut self, key: String, value: Box<Objects>) {
    self.objects.insert(key, value);
  }
}
