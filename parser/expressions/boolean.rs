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
  token: Token,
  value: bool,
}

impl Expression for Boolean {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      value: false,
    }
  }

  fn from_token(token: Token) -> Self {
    Self {
      token: token.clone(),
      value: token.token.expect_keyword(&Keywords::TRUE),
    }
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    self.get_token().value
  }
}

impl Boolean {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::BOOLEAN(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::BOOLEAN(Expression::from_token(token)))
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Box<Expressions> {
    Boolean::new_box_from_token(parser.get_current_token())
  }

  pub fn get_value(&self) -> bool {
    self.value.clone()
  }
}
