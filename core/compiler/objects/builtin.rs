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
  pub obj: Option<Box<Objects>>,
  pub fun: Option<BuiltInFn>,
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
