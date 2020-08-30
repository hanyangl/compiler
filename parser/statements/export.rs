use crate::Environment;
use crate::Parser;
use crate::tokens::{Token, Signs, TokenType};

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
  pub token: Token,
  pub value: Option<Box<Statements>>,
}

impl Statement for Export {
  fn new() -> Export {
    Export {
      token: Token::new_empty(),
      value: None,
    }
  }

  fn from_token(token: Token) -> Export {
    let mut export: Export = Statement::new();

    export.token = token;

    export
  }

  fn string(self) -> String {
    format!(
      "{} {};",
      self.token.value,
      match self.value {
        Some(value) => value.string(),
        None => String::new(),
      },
    )
  }
}

impl Export {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Statements>> {
    let mut export: Export = Statement::from_token(parser.current_token.clone());

    // Get the next token.
    parser.next_token();

    // Parse statement.
    export.value = parser.parse_statement(environment, standard_library);

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    Some(Box::new(Statements::EXPORT(export)))
  }
}
