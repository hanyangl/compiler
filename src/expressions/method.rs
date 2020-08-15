use crate::data::Token;
use crate::expressions::{Expressions, Expression, parse as expression_parse};
use crate::parser::Parser;

// EXPRESSION //
#[derive(Debug, Clone)]
pub struct Method {
  pub token: Token,
  left: Box<Expressions>,
  right: Box<Expressions>,
}

impl Expression for Method {
  fn new() -> Method {
    Method {
      token: Token::empty(),
      left: Box::new(Expressions::DEFAULT(Expression::new())),
      right: Box::new(Expressions::DEFAULT(Expression::new())),
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
