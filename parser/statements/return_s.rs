use crate::Environment;
use crate::expressions::{Expressions, parse as parse_expression};
use crate::{Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType, Types};

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
  pub token: Token,
  pub data_type: Token,
  pub value: Option<Box<Expressions>>,
}

impl Statement for Return {
  fn new() -> Return {
    Return {
      token: Token::new_empty(),
      data_type: Token::from_value(String::from("void"), 0, 0),
      value: None,
    }
  }

  fn from_token(token: Token) -> Return {
    let mut return_s: Return = Statement::new();

    return_s.token = token;

    return_s
  }

  fn string(self) -> String {
    format!(
      "{}{};",
      self.token.value,
      match self.value {
        Some(value) => format!(" {}", value.string()),
        None => String::new(),
      },
    )
  }
}

impl Return {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Statements>> {
    let mut return_s: Return = Statement::from_token(parser.current_token.clone());

    // Get the next token.
    parser.next_token();

    // Parse the value.
    return_s.value = parse_expression(parser, Precedence::LOWEST, environment, standard_library);

    // Parse value data type.
    match return_s.value.clone() {
      Some(value) => {
        return_s.data_type = Types::from_expression(value, environment);
      },
      None => {},
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    Some(Box::new(Statements::RETURN(return_s)))
  }
}
