use crate::data::Token;
use crate::expressions::Expression;
use crate::parser::{Parser, Expressions};
use crate::statements::expression;

#[derive(Debug, Clone)]
pub struct Infix {
  token: Token,
  left: Box<Expressions>,
  operator: String,
  right: Box<Expressions>,
}

impl Expression for Infix {
  fn new() -> Infix {
    Infix {
      token: Token::empty(),
      left: Box::new(Expressions::DEFAULT(Expression::new())),
      operator: String::new(),
      right: Box::new(Expressions::DEFAULT(Expression::new())),
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
      self.left.string(),
      self.operator,
      self.right.string()
    )
  }
}

pub fn parse<'a>(parser: &'a mut Parser, left: Box<Expressions>) -> Infix {
  let mut exp: Infix = Expression::from_token(&parser.current_token);

  exp.left = left;

  let precedence = parser.current_precedence();
  parser.next_token();

  exp.right = expression::parse(parser, precedence);

  exp
}
