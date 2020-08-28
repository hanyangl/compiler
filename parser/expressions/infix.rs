use crate::{Environment, Parser};
use crate::tokens::Token;

use super::{Expression, Expressions, parse as parse_expression};

#[derive(Debug, Clone, PartialEq)]
pub struct Infix {
  pub token: Token,
  pub left: Option<Box<Expressions>>,
  pub operator: String,
  pub right: Option<Box<Expressions>>,
}

impl Expression for Infix {
  fn new() -> Infix {
    Infix {
      token: Token::new_empty(),
      left: None,
      operator: String::new(),
      right: None,
    }
  }

  fn from_token(token: Token) -> Infix {
    Infix {
      token: token.clone(),
      left: None,
      operator: token.value,
      right: None,
    }
  }

  fn string(self) -> String {
    format!(
      "{} {} {}",
      match self.left {
        Some(left) => left.string(),
        None => String::new(),
      },
      self.operator,
      match self.right {
        Some(right) => right.string(),
        None => String::new(),
      }
    )
  }
}

impl Infix {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::INFIX(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::INFIX(Expression::from_token(token)))
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    left_expression: Option<Box<Expressions>>,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Box<Expressions> {
    let mut exp: Infix = Expression::from_token(parser.current_token.clone());

    // Set the left expression.
    exp.left = left_expression;

    // Get the current precedence.
    let precedence = parser.current_precedence();

    // Get the next token.
    parser.next_token();

    // Set the right expression.
    exp.right = parse_expression(parser, None, precedence, environment, standard_library);

    // Return the infix expression.
    Box::new(Expressions::INFIX(exp))
  }
}
