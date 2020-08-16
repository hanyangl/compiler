use crate::data::Token;
use crate::parser::Parser;

use super::{Expressions, Expression, parse as expression_parse, identifier::Identifier};

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct Method {
  pub token: Token,
  pub left: Box<Expressions>,
  pub right: Box<Expressions>,
}

impl Expression for Method {
  fn new() -> Method {
    Method {
      token: Token::empty(),
      left: Identifier::new(),
      right: Identifier::new(),
    }
  }

  fn from_token(token: &Token) -> Method {
    let mut exp: Method = Expression::new();

    exp.token = token.clone();

    exp
  }

  fn string(self) -> String {
    format!(
      "{}{}{}",
      self.left.string(),
      self.token.value,
      self.right.string(),
    )
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser, left: Option<Box<Expressions>>) -> Method {
  let mut exp: Method = Expression::from_token(&parser.current_token.clone());

  match left {
    Some(x) => {
      exp.left = x;
    },
    None => {},
  }

  let precedence = parser.current_precedence();
  parser.next_token();

  match expression_parse(parser, precedence) {
    Some(right) => {
      exp.right = right;
    },
    None => {},
  };

  exp
}
// END PARSER //
