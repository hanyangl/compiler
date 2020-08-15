use crate::data::Token;
use crate::expressions::{Expression, Expressions, parse as expression_parse};
use crate::parser::{Parser, precedence::Precedence};

#[derive(Debug, Clone)]
pub struct Prefix {
  pub token: Token,
  operator: String,
  right: Option<Box<Expressions>>,
}

impl Expression for Prefix {
  fn new() -> Prefix {
    Prefix {
      token: Token::empty(),
      operator: String::new(),
      right: None,
    }
  }

  fn from_token(token: &Token) -> Prefix {
    let mut expression: Prefix = Expression::new();

    expression.token = token.clone();
    expression.operator = token.value.clone();

    expression
  }

  fn string(self) -> String {
    format!(
      "({}{})",
      self.operator,
      match self.right {
        Some(x) => x.string(),
        None => "".to_string(),
      },
    )
  }
}

pub fn parse<'a>(parser: &'a mut Parser) -> Prefix {
  let mut exp: Prefix = Expression::from_token(&parser.current_token.clone());

  parser.next_token();

  exp.right = expression_parse(parser, Precedence::PREFIX);

  exp
}
