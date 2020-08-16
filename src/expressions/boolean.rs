use crate::data::{Token, Tokens, Types};
use crate::parser::Parser;

use super::Expression;

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
  pub token: Token,
  pub value: bool,
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
// END EXPRESSION //


// PARSE //
pub fn parse<'a>(parser: &'a mut Parser) -> Boolean {
  Expression::from_token(&parser.current_token.clone())
}
// END PARSE //
