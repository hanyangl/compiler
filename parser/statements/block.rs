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

  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Box<Statements> {
    let mut block: Block = Statement::from_token(parser.current_token.clone());

    // Get the next token.
    parser.next_token();

    // Parse block statements.
    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) &&
      !parser.current_token_is(Box::new(Tokens::EOF)) {
      // Parse statement
      match parser.parse_statement(environment) {
        Some(statement) => block.statements.push(statement),
        None => {},
      }

      // Get the next token.
      parser.next_token();
    }

    // Return the block statement.
    Box::new(Statements::BLOCK(block))
  }
}
