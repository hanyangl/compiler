use crate::Environment;
use crate::expressions::{Identifier, Expressions, parse as parse_expression};
use crate::{Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType};

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct Show {
  pub token: Token,
  pub value: Box<Expressions>,
}

impl Statement for Show {
  fn new() -> Show {
    Show {
      token: Token::new_empty(),
      value: Identifier::new_box(),
    }
  }

  fn from_token(token: Token) -> Show {
    Show {
      token,
      value: Identifier::new_box(),
    }
  }

  fn string(self) -> String {
    format!(
      "{}({});",
      self.token.value,
      self.value.string(),
    )
  }
}

impl Show {
  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Statements>> {
    let mut show: Show = Statement::from_token(parser.current_token.clone());

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      let line = parser.get_error_line_next_token();
      parser.errors.push(format!("{} expect `(`, got `{}` instead.", line, parser.next_token.value));
      return None;
    }

    // Get the next token.
    parser.next_token();

    // Parse expression.
    match parse_expression(parser, Precedence::LOWEST, environment, true) {
      Some(expression) => {
        show.value = expression;
      },
      None => {
        println!("TODO(show): Parse expression.");
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

    // Return the show statement.
    Some(Box::new(Statements::SHOW(show)))
  }
}
