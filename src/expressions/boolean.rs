use crate::data::{Token, Tokens, Types};
use crate::expressions::Expression;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Boolean {
  token: Token,
  value: bool,
}

impl Expression for Boolean {
  fn new() -> Boolean {
    Boolean {
      token: Token::empty(),
      value: false,
    }
  }

  fn from_token(token: &Token) -> Boolean {
    Boolean {
      token: token.clone(),
      value: token.token == Tokens::TYPE && token.data_type == Types::TRUE,
    }
  }

  fn string(self) -> String {
    self.token.value
  }
}

pub fn parse<'a>(parser: &'a mut Parser) -> Boolean {
  Expression::from_token(&parser.current_token.clone())
}
