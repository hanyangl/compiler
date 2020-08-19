use crate::tokens::Token;

use super::{Expression, Expressions};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Expression for Identifier {
  fn new() -> Identifier {
    Identifier {
      token: Token::new_empty(),
      value: String::new(),
    }
  }

  fn from_token(token: Token) -> Identifier {
    Identifier {
      token: token.clone(),
      value: token.value.clone(),
    }
  }

  fn string(self) -> String {
    self.value
  }
}

impl Identifier {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::IDENTIFIER(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::IDENTIFIER(Expression::from_token(token)))
  }
}
