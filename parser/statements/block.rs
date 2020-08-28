use crate::{Environment, Parser};
use crate::tokens::*;

use super::{Statements, Statement};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
  pub token: Token,
  pub statements: Vec<Box<Statements>>,
}

impl Statement for Block {
  fn new() -> Block {
    Block {
      token: Token::new_empty(),
      statements: Vec::new(),
    }
  }

  fn from_token(token: Token) -> Block {
    Block {
      token,
      statements: Vec::new(),
    }
  }

  fn string(self) -> String {
    let mut strings: Vec<String> = Vec::new();

    for stmt in self.statements {
      strings.push(stmt.string());
    }

    format!("{{\n{}\n}}", strings.join("\n"))
  }
}

impl Block {
  pub fn new_box() -> Box<Statements> {
    Box::new(Statements::BLOCK(Statement::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Statements> {
    Box::new(Statements::BLOCK(Statement::from_token(token)))
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    data_type: Token,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Statements>> {
    let mut block: Block = Statement::from_token(parser.current_token.clone());

    // Get the next token.
    parser.next_token();

    // Parse block statements.
    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) &&
      !parser.current_token_is(Box::new(Tokens::EOF)) {
      // Parse statement
      match parser.parse_statement(environment, standard_library) {
        Some(statement) => {
          // Parse return data type.
          match statement.clone().get_return() {
            Some(return_s) => {
              match data_type.token.clone().get_type() {
                Some(data_type_token) => {
                  if !return_s.data_type.token.expect_type(data_type_token) {
                    let line = parser.get_error_line(
                      return_s.token.line - 1,
                      return_s.token.position - 1,
                      return_s.token.value.len(),
                    );

                    parser.errors.push(format!("{} the return value not satisfied the {} type.", line, data_type.value));

                    return None;
                  }
                },
                None => {},
              }
            },
            None => {},
          }

          block.statements.push(statement);
        },
        None => {},
      }

      // Get the next token.
      parser.next_token();
    }

    // Return the block statement.
    Some(Box::new(Statements::BLOCK(block)))
  }
}
