use crate::tokens::Token;

use super::{
  Expression,
  Expressions,
};

#[derive(Debug, Clone, PartialEq)]
pub struct StringE {
  pub token: Token,
  pub value: String,
}

impl Expression for StringE {
  fn new() -> StringE {
    StringE {
      token: Token::new_empty(),
      value: String::new(),
    }
  }

  fn from_token(token: Token) -> StringE {
    StringE {
      token: token.clone(),
      value: token.value.clone(),
    }
  }

  fn string(self) -> String {
    self.value
  }
}

impl StringE {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::STRING(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::STRING(Expression::from_token(token)))
  }
}
