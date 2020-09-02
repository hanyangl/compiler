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
  pub token: Token,
  pub name: Token,
  pub data_type: Token,
  pub value: Option<Box<Expressions>>,
}

impl Statement for Variable {
  fn new() -> Variable {
    Variable {
      token: Token::new_empty(),
      name: Token::new_empty(),
      data_type: Token::from_value("any", 0, 0),
      value: None,
    }
  }

  fn from_token(token: Token) -> Variable {
    let mut variable: Variable = Statement::new();

    variable.token = token;

    variable
  }

  fn string(self) -> String {
    let mut value = String::new();

    if let Some(default_value) = self.value {
      let default_value = default_value.string();

      if default_value.ends_with(";") {
        value = default_value[..default_value.len() - 1].to_string();
      } else {
        value = default_value;
      }
    }

    format!(
      "{} {}: {} = {};",
      self.token.value,
      self.name.value,
      self.data_type.value,
      value,
    )
  }
}

impl Variable {
  pub fn new_box() -> Box<Statements> {
    Box::new(Statements::VARIABLE(Statement::new()))
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut variable: Variable = Statement::from_token(parser.current_token.clone());

    // Check if the next token is a valid identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      let mut message = format!("`{}` is not a valid variable name.", parser.next_token.value.clone());

      if parser.next_token_is(Signs::new(Signs::COLON)) {
        message = String::from("you must enter the variable name.");
      }

      return Err(Error::from_token(
        message,
        parser.next_token.clone(),
      ));
    }

    // Set the variable name.
    variable.name = parser.current_token.clone();

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
          format!("expect `:`, got `{}` instead.", parser.next_token.value.clone()),
          parser.next_token.clone(),
        ));
      }

      // Get the next token.
      parser.next_token();

      // Parse type.
      match parse_type(parser, false) {
        Ok(data_type) => {
          // Set the variable type.
          variable.data_type = data_type.clone();

          // Check if the next token is an assign sign.
          if !parser.expect_token(Signs::new(Signs::ASSIGN)) {
            return Err(Error::from_token(
              format!("expect `=`, got `{}` instead.", parser.next_token.value.clone()),
              parser.next_token.clone(),
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
            parser.current_token.clone(),
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
