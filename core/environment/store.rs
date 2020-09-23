use crate::{
  compiler::Objects,
  typechecker::TTypes,
};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Store {
  consts: Vec<String>,

  objects: HashMap<String, Box<Objects>>,
  types: HashMap<String, TTypes>,

  outer: Option<Box<Store>>,
}

impl Store {
  pub fn new() -> Self {
    Self {
      consts: Vec::new(),

      objects: HashMap::new(),
      types: HashMap::new(),

      outer: None,
    }
  }

  pub fn new_box() -> Box<Self> {
    Box::new(Self::new())
  }

  pub fn from_store(outer: &Store) -> Box<Self> {
    let mut store: Self = Self::new();

    store.outer = Some(Box::new(outer.clone()));

    Box::new(store)
  }

  pub fn get_consts(&self) -> Vec<String> {
    self.consts.clone()
  }

  pub fn has_const(&mut self, name: &String) -> bool {
    self.consts.contains(name)
  }

  pub fn set_const(&mut self, name: String) {
    self.consts.push(name)
  }

  pub fn has_object(&self, key: &String) -> bool {
    self.objects.contains_key(key)
  }

  pub fn is_in_outer(&self, key: &String) -> bool {
    match self.get_outer() {
      Some(outer) => if outer.has_object(key) { true } else { outer.is_in_outer(key) },
      None => false,
    }
  }

  pub fn get_object(&self, key: &String) -> Option<Box<Objects>> {
    match self.objects.get(key) {
      Some(object) => Some(object.clone()),
      None => match self.get_outer() {
        Some(outer) => outer.get_object(key),
        None => None,
      },
    }
  }

  pub fn set_object(&mut self, key: String, value: Box<Objects>) {
    self.objects.insert(key, value);
  }

  pub fn delete_object(&mut self, key: &String) -> Option<Box<Objects>> {
    self.objects.remove(key)
  }

  pub fn replace_object(&mut self, key: &String, new_value: Box<Objects>) {
    if self.has_object(key) {
      self.delete_object(key);
      self.set_object(key.clone(), new_value);
    } else if let Some(outer) = self.get_outer().as_mut() {
      outer.replace_object(key, new_value);

      self.outer = Some(outer.clone());
    }
  }

  pub fn get_type(&self, key: &String) -> Option<TTypes> {
    match self.types.get(key) {
      Some(data_type) => Some(data_type.clone()),
      None => match self.get_outer() {
        Some(outer) => outer.get_type(key),
        None => None,
      },
    }
  }

  pub fn set_type(&mut self, key: String, value: TTypes) {
    self.types.insert(key, value);
  }

  pub fn get_outer(&self) -> Option<Box<Store>> {
    self.outer.clone()
  }
}
