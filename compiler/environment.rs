use std::collections::HashMap;

use super::objects::Objects;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
  pub store: HashMap<String, Box<Objects>>,
  pub outer: Option<Box<Environment>>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      store: HashMap::new(),
      outer: None,
    }
  }

  pub fn from_environment(environment: Environment) -> Environment {
    Environment {
      store: HashMap::new(),
      outer: Some(Box::new(environment)),
    }
  }

  pub fn get(&self, name: String) -> Option<Box<Objects>> {
    match self.store.get(&name) {
      Some(object) => Some(object.clone()),
      None => match &self.outer {
        Some(outer) => outer.get(name),
        None => None,
      },
    }
  }

  pub fn has(&mut self, name: String) -> bool {
    match self.get(name) {
      Some(_) => true,
      None => false,
    }
  }

  pub fn set(&mut self, name: String, object: Box<Objects>) {
    self.store.insert(name, object);
  }
}
