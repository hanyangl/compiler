use crate::objects::Objects;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
  pub store: HashMap<String, Box<Objects>>,
  outer: Option<Box<Environment>>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      store: HashMap::new(),
      outer: None,
    }
  }

  pub fn from_environment(env: Environment) -> Environment {
    let mut env_r: Environment = Environment::new();
    env_r.outer = Some(Box::new(env));

    env_r
  }

  pub fn get_first(&self, name: String) -> Option<Box<Objects>> {
    match self.store.get(&name) {
      Some(object) => Some(object.clone()),
      None => None,
    }
  }

  pub fn get(&self, name: String) -> Option<Box<Objects>> {
    match self.get_first(name.clone()) {
      Some(object) => Some(object),
      None => match self.outer.clone() {
        Some(env) => env.get(name),
        None => None,
      },
    }
  }

  pub fn set(&mut self, name: String, value: Box<Objects>) -> Box<Objects> {
    self.store.insert(name, value.clone());

    value
  }
}
