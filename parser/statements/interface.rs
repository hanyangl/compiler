use crate::{
  Error,
  parse_type,
  Parser,
  tokens::{
    Signs,
    Token,
    Tokens,
  },
};

use super::{
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceMethod {
  pub token: Token,
  pub data_type: Token,
}

impl InterfaceMethod {
  pub fn string(self) -> String {
    format!(
      "{}: {}",
      self.token.value,
      self.data_type.value,
    )
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Interface {
  pub token: Token,
  pub name: Token,

  pub methods: Vec<InterfaceMethod>,
}

impl Statement for Interface {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      name: Token::new_empty(),
      
      methods: Vec::new(),
    }
  }

  fn from_token(token: Token) -> Self {
    let mut interface: Self = Statement::new();

    interface.token = token;

    interface
  }

  fn string(self) -> String {
    let mut methods: Vec<String> = Vec::new();

    for method in self.methods {
      methods.push(method.string());
    }

    format!(
      "{} {} {{\n {} \n}}",
      self.token.value,
      self.name.value,
      methods.join(";\n"),
    )
  }
}

impl Interface {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    _standard_library: bool,
    _with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut interface: Self = Statement::from_token(parser.current_token.clone());

    // Check if the next token is an identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      return Err(Error::from_token(
        format!("`{}` is not a valid identifier.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    // Set the current token as the interface name.
    interface.name = parser.current_token.clone();

    // Check if the next token is a left brace.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) {
      // Check if the next token is an identifier.
      if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
        return Err(Error::from_token(
          format!("`{}` is not a valid identifier.", parser.next_token.value.clone()),
          parser.next_token.clone(),
        ));
      }

      let key = parser.current_token.clone();

      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        return Err(Error::from_token(
          format!("expect `:`, got `{}` instead.", parser.next_token.value.clone()),
          parser.next_token.clone(),
        ));
      }

      // Get the next token.
      parser.next_token();

      // Parse type.
      match parse_type(parser) {
        Ok(data_type) => {
          // Add  the method to the interface.
          interface.methods.push(InterfaceMethod {
            token: key,
            data_type,
          });
        },
        Err(_) => {
          return Err(Error::from_token(
            format!("`{}` is not a valid data type.", parser.current_token.value.clone()),
            parser.current_token.clone(),
          ));
        },
      }

      // Check if the next token is a comma or semicolon.
      if parser.next_token_is(Signs::new(Signs::COMMA)) ||
        parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
        // Get the next token.
        parser.next_token();
      }

      // Get the next token.
      parser.next_token();
    }

    Ok(Box::new(Statements::INTERFACE(interface)))
  }
}
