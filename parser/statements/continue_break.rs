use crate::{
  Parser,
  Statement,
  Statements,
  tokens::{
    Signs,
    Token,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ContinueBreak {
  token: Token,
}

impl Statement for ContinueBreak {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
    }
  }

  fn from_token(token: Token) -> Self {
    Self {
      token
    }
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    format!("{};", self.get_token().value)
  }
}

impl ContinueBreak {
  pub fn new_box() -> Box<Statements> {
    Box::new(Statements::CONTINUEBREAK(Statement::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Statements> {
    Box::new(Statements::CONTINUEBREAK(Statement::from_token(token)))
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Box<Statements> {
    let continue_break = Self::new_box_from_token(parser.get_current_token());

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    continue_break
  }
}
