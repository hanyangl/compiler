use crate::{
  Parser,
  tokens::{
    Keywords,
    Token,
  },
};

use super::{
  Expression,
  Expressions,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
  pub token: Token,
  pub value: bool,
}

impl Expression for Boolean {
  fn new() -> Boolean {
    Boolean {
      token: Token::new_empty(),
      value: false,
    }
  }

  fn from_token(token: Token) -> Boolean {
    Boolean {
      token: token.clone(),
      value: token.token.expect_keyword(Keywords::TRUE),
    }
  }

  fn string(self) -> String {
    self.token.value
  }
}

impl Boolean {
  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::BOOLEAN(Expression::from_token(token)))
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Box<Expressions> {
    Boolean::new_box_from_token(parser.current_token.clone())
  }
}
