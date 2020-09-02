use crate::compiler::Objects;

use sflyn_parser::{
  Expressions,
  tokens::Token,
};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
  pub objects: HashMap<String, Box<Objects>>,

  pub types: HashMap<String, Token>,
  pub interfaces: HashMap<String, Token>,
  pub function_arguments: HashMap<String, Vec<Box<Expressions>>>,

  pub outer: Option<Box<Store>>,
}

impl Store {
  pub fn new() -> Self {
    Self {
      objects: HashMap::new(),

      types: HashMap::new(),
      interfaces: HashMap::new(),
      function_arguments: HashMap::new(),

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

  pub fn get_type(&self, key: String) -> Option<Token> {
    match self.types.get(&key) {
      Some(data_type) => Some(data_type.clone()),
      None => match &self.outer {
        Some(outer) => outer.get_type(key),
        None => None,
      },
    }
  }

  pub fn set_type(&mut self, key: String, value: Token) {
    self.types.insert(key, value);
  }

  pub fn get_interface(&self, key: String) -> Option<Token> {
    match self.interfaces.get(&key) {
      Some(interface) => Some(interface.clone()),
      None => match &self.outer {
        Some(outer) => outer.get_interface(key),
        None => None,
      },
    }
  }

  pub fn set_interface(&mut self, key: String, value: Token) {
    self.interfaces.insert(key, value);
  }

  pub fn get_function_arguments(&self, key: String) -> Option<Vec<Box<Expressions>>> {
    match self.function_arguments.get(&key) {
      Some(function_arguments) => Some(function_arguments.clone()),
      None => match &self.outer {
        Some(outer) => outer.get_function_arguments(key),
        None => None,
      },
    }
  }

  pub fn set_function_arguments(&mut self, key: String, value: Vec<Box<Expressions>>) {
    self.function_arguments.insert(key, value);
  }
}
