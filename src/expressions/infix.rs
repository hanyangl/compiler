use crate::data::Token;
use crate::expressions::{Expression, Expressions, parse as expression_parse};
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Infix {
  pub token: Token,
  left: Option<Box<Expressions>>,
  operator: String,
  right: Option<Box<Expressions>>,
}

impl Expression for Infix {
  fn new() -> Infix {
    Infix {
      token: Token::empty(),
      left: None,
      operator: String::new(),
      right: None,
    }
  }

  fn from_token(token: &Token) -> Infix {
    let mut exp: Infix = Expression::new();

    exp.token = token.clone();
    exp.operator = token.value.clone();

    exp
  }

  fn string(self) -> String {
    format!(
      "({} {} {})",
      match self.left {
        Some(x) => x.string(),
        None => "".to_string(),
      },
      self.operator,
      match self.right {
        Some(x) => x.string(),
        None => "".to_string(),
      },
    )
  }
}

pub fn parse<'a>(parser: &'a mut Parser, left: Option<Box<Expressions>>) -> Infix {
  let mut exp: Infix = Expression::from_token(&parser.current_token);

  exp.left = left;

  let precedence = parser.current_precedence();
  parser.next_token();

  exp.right = expression_parse(parser, precedence);

  exp
}
