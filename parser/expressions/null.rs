use crate::tokens::Token;

use super::{
  Expression,
  Expressions,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Null {
  pub token: Token,
}

impl Expression for Null {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
    }
  }

  fn from_token(token: Token) -> Self {
    Self {
      token,
    }
  }

  fn string(self) -> String {
    self.token.value
  }
}

impl Null {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::NULL(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::NULL(Expression::from_token(token)))
  }
}
