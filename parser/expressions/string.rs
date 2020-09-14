use crate::tokens::Token;

use super::{
  Expression,
  Expressions,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StringE {
  token: Token,
  value: String,
}

impl Expression for StringE {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      value: String::new(),
    }
  }

  fn from_token(token: Token) -> Self {
    Self {
      token: token.clone(),
      value: token.value.clone(),
    }
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    self.get_value()
  }
}

impl StringE {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::STRING(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::STRING(Expression::from_token(token)))
  }

  pub fn get_value(&self) -> String {
    self.value.clone()
  }
}
