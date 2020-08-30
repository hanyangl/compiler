use crate::Environment;
use crate::expressions::{Expressions, StringE, Identifier};
use crate::{modules, Parser};
use crate::tokens::*;

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
  pub token: Token,
  pub requires: Vec<Box<Expressions>>,
  pub path: Box<Expressions>,
}

impl Statement for Import {
  fn new() -> Import {
    Import {
      token: Token::new_empty(),
      requires: Vec::new(),
      path: StringE::new_box(),
    }
  }

  fn from_token(token: Token) -> Import {
    let mut import: Import = Statement::new();

    import.token = token;

    import
  }

  fn string(self) -> String {
    let mut requires: Vec<String> = Vec::new();

    for require in self.requires {
      requires.push(require.string());
    }

    format!(
      "{} {} from {};",
      self.token.value,
      if requires.len() == 1 {
        requires[0].clone()
      } else {
        format!("{{ {} }}", requires.join(", "))
      },
      self.path.string(),
    )
  }
}

impl Import {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Statements>> {
    let mut import: Import = Statement::from_token(parser.current_token.clone());

    // Check if the next token is a left brace.
    if parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      // Get the next token.
      parser.next_token();

      while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) {
        // Check if the current token is an identifier.
        if !parser.current_token_is(Box::new(Tokens::IDENTIFIER)) {
          let line = parser.get_error_line_current_token();
          parser.errors.push(format!("{} is not a valid identifier.", line));
          return None;
        }

        import.requires.push(Identifier::new_box_from_token(parser.current_token.clone()));

        // Check if the next token is a comma.
        if parser.next_token_is(Signs::new(Signs::COMMA)) {
          // Get the next token.
          parser.next_token();
        }

        // Get the next token.
        parser.next_token();
      }
    } else {
      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is `from`.
    if !parser.expect_token(Keywords::new(Keywords::FROM)) {
      let line = parser.get_error_line_next_token();
      parser.errors.push(format!("{} expect `from`, got `{}` instead,", line, parser.next_token.value));
      return None;
    }

    // Check if the next token is a string.
    if !parser.expect_token(Box::new(Tokens::STRING)) {
      let line = parser.get_error_line_next_token();
      parser.errors.push(format!("{} is not a valid string.", line));
      return None;
    }

    import.path = StringE::new_box_from_token(parser.current_token.clone());

    if !modules::resolve_path_expression(parser, import.clone(), environment, standard_library) {
      return None;
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    Some(Box::new(Statements::IMPORT(import)))
  }
}
