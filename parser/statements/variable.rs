use crate::{
  Error,
  Expressions,
  parse_expression,
  parse_type,
  Parser,
  Precedence,
  tokens::*,
};

use super::{
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
  token: Token,
  name: Token,
  data_type: Token,
  value: Option<Box<Expressions>>,
}

impl Statement for Variable {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      name: Token::new_empty(),
      data_type: Token::from_value("any", 0, 0),
      value: None,
    }
  }

  fn from_token(token: Token) -> Self {
    let mut variable: Self = Statement::new();

    variable.token = token;

    variable
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    let mut value = String::new();

    if let Some(default_value) = self.get_value() {
      let default_value = default_value.string();

      if default_value.ends_with(";") {
        value = default_value[..default_value.len() - 1].to_string();
      } else {
        value = default_value;
      }
    }

    format!(
      "{} {}: {} = {};",
      self.get_token().value,
      self.get_name().value,
      self.get_type().value,
      value,
    )
  }
}

impl Variable {
  pub fn new_box() -> Box<Statements> {
    Box::new(Statements::VARIABLE(Statement::new()))
  }

  pub fn get_name(&self) -> Token {
    self.name.clone()
  }

  pub fn get_type(&self) -> Token {
    self.data_type.clone()
  }

  pub fn get_value(&self) -> Option<Box<Expressions>> {
    self.value.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut variable: Variable = Statement::from_token(parser.get_current_token());

    // Check if the next token is a valid identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      let mut message = format!("`{}` is not a valid variable name.", parser.get_next_token().value);

      if parser.next_token_is(Signs::new(Signs::COLON)) {
        message = String::from("you must enter the variable name.");
      }

      return Err(Error::from_token(
        message,
        parser.get_next_token(),
      ));
    }

    // Set the variable name.
    variable.name = parser.get_current_token();

    // Check if the next token is an assign sign.
    if parser.next_token_is(Signs::new(Signs::ASSIGN)) {
      // Get the next token.
      parser.next_token();

      // Get the next token.
      parser.next_token();

      // Parse current token (Variable value).
      match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
        Ok(exp) => {
          variable.value = Some(exp);
        },
        Err(error) => {
          return Err(error);
        },
      }
    } else {
      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        return Err(Error::from_token(
          format!("expect `:`, got `{}` instead.", parser.get_next_token().value),
          parser.get_next_token(),
        ));
      }

      // Get the next token.
      parser.next_token();

      // Parse type.
      match parse_type(parser) {
        Ok(data_type) => {
          // Set the variable type.
          variable.data_type = data_type.clone();

          // Check if the next token is an assign sign.
          if !parser.expect_token(Signs::new(Signs::ASSIGN)) {
            return Err(Error::from_token(
              format!("expect `=`, got `{}` instead.", parser.get_next_token().value),
              parser.get_next_token(),
            ));
          }

          // Get the next token.
          parser.next_token();

          // Parse current token (Variable value).
          match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
            Ok(exp) => {
              variable.value = Some(exp);
            },
            Err(error) => {
              return Err(error);
            },
          }
        },
        Err(_) => {
          return Err(Error::from_token(
            String::from("is not a valid type."),
            parser.get_current_token(),
          ));
        },
      }
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    // Return the statement.
    Ok(Box::new(Statements::VARIABLE(variable.clone())))
  }
}
