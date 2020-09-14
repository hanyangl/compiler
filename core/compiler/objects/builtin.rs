use std::fmt;

use sflyn_parser::tokens::Token;

use super::{
  Object,
  Objects,
};

type BuiltInFn = fn(
  token: Token,
  arguments: Vec<Box<Objects>>,
) -> Box<Objects>;

#[derive(Clone)]
pub struct BuiltIn {
  obj: Option<Box<Objects>>,
  fun: Option<BuiltInFn>,
}

impl fmt::Debug for BuiltIn {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "builtin")
  }
}

impl PartialEq for BuiltIn {
  fn eq(&self, _other: &Self) -> bool {
    false
  }
}

impl Object for BuiltIn {
  fn string(&self) -> String {
    String::from("builtin")
  }
}

impl BuiltIn {
  pub fn new(obj: Option<Box<Objects>>, fun: Option<BuiltInFn>) -> Self {
    Self { obj, fun }
  }

  pub fn new_box(obj: Option<Box<Objects>>, fun: Option<BuiltInFn>) -> Box<Objects> {
    Box::new(Objects::BUILTIN(Self::new(obj, fun)))
  }

  pub fn get_object(&self) -> Option<Box<Objects>> {
    self.obj.clone()
  }

  pub fn get_function(&self) -> Option<BuiltInFn> {
    self.fun.clone()
  }
}
