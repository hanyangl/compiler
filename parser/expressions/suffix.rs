use crate::{
  Parser,
  tokens::Token,
};

use super::{
  Expression,
  Expressions,
  Identifier,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Suffix {
  left: Box<Expressions>,
  token: Token,
}

impl Expression for Suffix {
  fn new() -> Self {
    Self {
      left: Identifier::new_box(),
      token: Token::new_empty(),
    }
  }

  fn from_token(token: Token) -> Self {
    Self {
      left: Identifier::new_box(),
      token,
    }
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    format!(
      "{}{}",
      self.get_left().string(),
      self.get_token().value,
    )
  }
}

impl Suffix {
  pub fn get_left(&self) -> Box<Expressions> {
    self.left.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    left_expression: Box<Expressions>,
  ) -> Box<Expressions> {
    let mut suffix: Self = Expression::from_token(parser.get_current_token());

    // Set the left expression.
    suffix.left = left_expression;

    // Return the suffix expression.
    Box::new(Expressions::SUFFIX(suffix))
  }
}
