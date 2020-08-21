use crate::{Parser, Precedence};
use crate::tokens::Token;

use super::{Expression, Expressions, parse as parse_expression};

#[derive(Debug, Clone, PartialEq)]
pub struct Prefix {
  pub token: Token,
  pub operator: String,
  pub right: Option<Box<Expressions>>,
}

impl Expression for Prefix {
  fn new() -> Prefix {
    Prefix {
      token: Token::new_empty(),
      operator: String::new(),
      right: None,
    }
  }

  fn from_token(token: Token) -> Prefix {
    Prefix {
      token: token.clone(),
      operator: token.value,
      right: None,
    }
  }

  fn string(self) -> String {
    format!(
      "{}{}",
      self.operator,
      match self.right {
        Some(right) => right.string(),
        None => String::new(),
      },
    )
  }
}

impl Prefix {
  pub fn parse<'a>(parser: &'a mut Parser) -> Box<Expressions> {
    let mut prefix: Prefix = Expression::from_token(parser.current_token.clone());

    // Get the next token.
    parser.next_token();

    // Parse the right expression.
    prefix.right = parse_expression(parser, Precedence::PREFIX);

    // Return the prefix expression.
    Box::new(Expressions::PREFIX(prefix))
  }
}
