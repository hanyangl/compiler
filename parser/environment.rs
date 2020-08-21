use std::collections::HashMap;

use super::expressions::Expressions;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
  pub store: HashMap<String, Box<Expressions>>,
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

  pub fn get_first(&self, name: String) -> Option<Box<Expressions>> {
    match self.store.get(&name) {
      Some(expression) => Some(expression.clone()),
      None => None,
    }
  }

  pub fn get(&self, name: String) -> Option<Box<Expressions>> {
    match self.get_first(name.clone()) {
      Some(expression) => Some(expression),
      None => match self.outer.clone() {
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

  pub fn set(&mut self, name: String, expression: Box<Expressions>) {
    self.store.insert(name, expression);
  }
}
