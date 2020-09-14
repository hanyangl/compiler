use crate::{
  Error,
  Parser,
  tokens::*,
};

use super::{
  parse_statement,
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
  token: Token,
  statements: Vec<Box<Statements>>,
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

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    let mut strings: Vec<String> = Vec::new();

    for stmt in self.get_statements().iter() {
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

  pub fn get_statements(&self) -> Vec<Box<Statements>> {
    self.statements.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    from_class: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut block: Block = Statement::from_token(parser.get_current_token());

    // Get the next token.
    parser.next_token();

    // Parse block statements.
    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) &&
      !parser.current_token_is(Box::new(Tokens::EOF)) {
      // Parse statement
      match parse_statement(parser, standard_library, from_class, with_this) {
        Ok(statement) => {
          block.statements.push(statement);
        },
        Err(error) => {
          return Err(error);
        }
      }

      // Get the next token.
      parser.next_token();
    }

    // Return the block statement.
    Ok(Box::new(Statements::BLOCK(block)))
  }
}
