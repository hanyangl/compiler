use crate::data::Token;
use crate::expressions::Expression;
use crate::parser::{Parser, Expressions, precedence::Precedence};
use crate::statements::expression;

#[derive(Debug, Clone)]
pub struct Prefix {
  token: Token,
  operator: String,
  right: Box<Expressions>,
}

impl Expression for Prefix {
  fn new() -> Prefix {
    Prefix {
      token: Token::empty(),
      operator: String::new(),
      right: Box::new(Expressions::DEFAULT(Expression::new())),
    }
  }

  fn from_token(token: &Token) -> Prefix {
    let mut expression: Prefix = Expression::new();

    expression.token = token.clone();
    expression.operator = token.value.clone();

    expression
  }

  fn string(self) -> String {
    format!("({}{})", self.operator, self.right.string())
  }
}

pub fn parser<'a>(parser: &'a mut Parser) -> Prefix {
  let mut exp: Prefix = Expression::from_token(&parser.current_token.clone());

  parser.next_token();

  exp.right = expression::parse(parser, Precedence::PREFIX);

  exp
}
