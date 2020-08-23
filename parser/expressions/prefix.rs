use crate::{Environment, Parser, Precedence};
use crate::tokens::{Token, Types, Signs};

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
  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Expressions>> {
    let mut prefix: Prefix = Expression::from_token(parser.current_token.clone());

    // Get the next token.
    parser.next_token();

    // Parse the right expression.
    prefix.right = parse_expression(parser, Precedence::PREFIX, environment);

    match prefix.right.clone() {
      Some(right_exp) => {
        let data_type = Types::from_expression(right_exp.clone(), environment);

        let line = parser.get_error_line(
          right_exp.clone().token().line - 1,
          right_exp.clone().token().position - 1,
          right_exp.clone().string().len()
        );

        // Parse negation prefix.
        if prefix.token.token.clone().is_sign(Signs::NEGATION) && !data_type.token.clone().is_type(Types::BOOLEAN) {
          parser.errors.push(format!("{} `{}` not satisfied the boolean type.", line, right_exp.string()));

          return None;
        }

        // Parse minus prefix.
        if prefix.token.token.clone().is_sign(Signs::MINUS) && !data_type.token.clone().is_type(Types::NUMBER) {
          parser.errors.push(format!("{} `{}` not satisfied the number type.", line, right_exp.string()));

          return None;
        }
      },
      None => {},
    }

    // Return the prefix expression.
    Some(Box::new(Expressions::PREFIX(prefix)))
  }
}
