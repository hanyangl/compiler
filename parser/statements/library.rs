use crate::Environment;
use crate::expressions::{Identifier, Expressions, parse as parse_expression};
use crate::{Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType};

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct Library {
  pub token: Token,
  pub option: Box<Expressions>,
  pub value: Box<Expressions>,
}

impl Statement for Library {
  fn new() -> Library {
    Library {
      token: Token::new_empty(),
      option: Identifier::new_box(),
      value: Identifier::new_box(),
    }
  }

  fn from_token(token: Token) -> Library {
    let mut library: Library = Statement::new();

    library.token = token;

    library
  }

  fn string(self) -> String {
    format!(
      "{}({}, {});",
      self.token.value,
      self.option.string(),
      self.value.string(),
    )
  }
}

impl Library {
  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Statements>> {
    let mut library: Library = Statement::from_token(parser.current_token.clone());

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      let line = parser.get_error_line_next_token();
      parser.errors.push(format!("{} expect `(`, got `{}` instead.", line, parser.next_token.value));
      return None;
    }

    // Get the next token.
    parser.next_token();

    // Parse expression.
    match parse_expression(parser, None, Precedence::LOWEST, environment, true) {
      Some(expression) => {
        library.option = expression;
      },
      None => {
        println!("TODO(library): Parse option expression.");
        return None;
      },
    }

    // Check if the next token is a comma.
    if !parser.expect_token(Signs::new(Signs::COMMA)) {
      let line = parser.get_error_line_next_token();
      parser.errors.push(format!("{} expect `,`, got `{}` instead.", line, parser.next_token.value));
      return None;
    }

    // Get the next token.
    parser.next_token();

    // Parse expression.
    match parse_expression(parser, None, Precedence::LOWEST, environment, true) {
      Some(expression) => {
        library.value = expression;
      },
      None => {
        println!("TODO(library): Parse value expression.");
        return None;
      },
    }

    // Check if the next token is a right parentheses.
    if parser.next_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    // Return the library statement.
    Some(Box::new(Statements::LIBRARY(library)))
  }
}
