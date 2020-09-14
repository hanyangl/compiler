use crate::{
  Error,
  Expressions,
  parse_expression,
  Parser,
  Precedence,
  tokens::{
    Signs,
    Token,
  },
};

use super::{
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
  token: Token,
  value: Option<Box<Expressions>>,
}

impl Statement for Return {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      value: None,
    }
  }

  fn from_token(token: Token) -> Self {
    let mut return_s: Self = Statement::new();

    return_s.token = token;

    return_s
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    format!(
      "{}{};",
      self.get_token().value,
      match &self.value {
        Some(value) => format!(" {}", value.string()),
        None => String::new(),
      },
    )
  }
}

impl Return {
  pub fn get_value(&self) -> Option<Box<Expressions>> {
    self.value.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut return_s: Return = Statement::from_token(parser.get_current_token());

    // Get the next token.
    parser.next_token();

    // Check if the current token is not a semicolon.
    if !parser.current_token_is(Signs::new(Signs::SEMICOLON)) {
      // Parse the value.
      match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
        Ok(value) => {
          return_s.value = Some(value);
        },
        Err(error) => {
          return Err(error);
        },
      }
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    Ok(Box::new(Statements::RETURN(return_s)))
  }
}
