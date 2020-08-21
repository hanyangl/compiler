use crate::Environment;
use crate::expressions::{Expressions, Identifier, parse as parse_expression};
use crate::{Parser, Precedence};
use crate::tokens::*;
use crate::types::expression_is_type;

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
  pub token: Token,
  pub name: Box<Expressions>,
  pub data_type: Token,
  pub value: Option<Box<Expressions>>,
}

impl Statement for Variable {
  fn new() -> Variable {
    Variable {
      token: Token::new_empty(),
      name: Identifier::new_box(),
      data_type: Token::new_empty(),
      value: None,
    }
  }

  fn from_token(token: Token) -> Variable {
    let mut variable: Variable = Statement::new();

    variable.token = token;

    variable
  }

  fn string(self) -> String {
    format!(
      "{} {}: {} = {};",
      self.token.value,
      self.name.string(),
      self.data_type.value,
      match self.value {
        Some(value) => value.string(),
        None => String::new(),
      },
    )
  }
}

impl Variable {
  pub fn new_box() -> Box<Statements> {
    Box::new(Statements::VARIABLE(Statement::new()))
  }

  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Statements>> {
    let mut variable: Variable = Statement::from_token(parser.current_token.clone());

    // Check if the next token is a valid identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      let line = parser.get_error_line_next_token();
      let mut message = format!("{} `{}` is not a valid variable name.", line, parser.next_token.value.clone());

      if parser.next_token.token.clone().is_sign(Signs::COLON) {
        message = format!("{} you must enter the variable name.", line);
      }

      parser.errors.push(message);

      return None;
    }

    // Set the variable name.
    variable.name = Identifier::new_box_from_token(parser.current_token.clone());

    // Check if the next token is an assign sin.
    if parser.next_token_is(Signs::new(Signs::ASSIGN)) {
      // Get the next token.
      parser.next_token();

      // Get the next token.
      parser.next_token();

      // Parse current token (Variable value).
      match parse_expression(parser, Precedence::LOWEST) {
        Some(exp) => {
          variable.data_type = Types::from_expression(exp.clone());

          if variable.data_type.token.clone().is_illegal() {
            let mut line = parser.get_error_line_current_token();

            // Parse infix.
            match exp.clone().get_infix() {
              Some(infix) => match infix.left.clone() {
                Some(left) => {
                  line = parser.get_error_line(left.token().position - 1, exp.clone().string().len());
                },
                None => {},
              },
              None => {},
            }

            // Parse prefix.
            match exp.clone().get_prefix() {
              Some(prefix) => {
                line = parser.get_error_line(prefix.token.position - 1, exp.clone().string().len());
              },
              None => {},
            }

            // Add error to parser.
            parser.errors.push(format!("{} is not a valid expression.", line));

            return None;
          }

          variable.value = Some(exp);
        },
        None => {},
      }
    } else {
      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        let line = parser.get_error_line_next_token();

        parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.next_token.value.clone()));

        return None;
      }

      // Check if the next token is a valid type.
      match parser.next_token.token.clone().get_type() {
        Some(data_type) => {
          // Get the next token.
          parser.next_token();

          // Set the variable type.
          let data_type_token = parser.current_token.clone();
          variable.data_type = data_type_token.clone();

          // Check if the next token is an assign sign.
          if !parser.expect_token(Signs::new(Signs::ASSIGN)) {
            let line = parser.get_error_line_next_token();

            parser.errors.push(format!("{} expect `=`, got `{}` instead.", line, parser.next_token.value.clone()));

            return None;
          }

          // Get the next token.
          parser.next_token();

          // Parse current token (Variable value).
          match parse_expression(parser, Precedence::LOWEST) {
            Some(exp) => {
              if !expression_is_type(data_type.clone(), exp.clone()) {
                let line = parser.get_error_line_current_token();

                parser.errors.push(format!("{} `{}` not satisfied the {} type.", line, parser.current_token.value, data_type_token.value));

                return None;
              }

              variable.value = Some(exp);
            },
            None => {},
          }
        },
        None => {
          let line = parser.get_error_line_next_token();

          parser.errors.push(format!("{} `{}` is not a valid type.", line, parser.next_token.value.clone()));

          return None;
        },
      }
    }

    // Check if the name is used.
    if environment.has(variable.name.clone().string()) {
        let line = parser.get_error_line(variable.name.clone().token().position - 1, variable.name.clone().string().len());

        parser.errors.push(format!("{} `{}` is already in use.", line, variable.name.clone().string()));

        return None;
    }

    // Set the expression to the environment.
    match variable.value.clone() {
        Some(value) => {
            environment.set(variable.name.clone().string(), value.clone());
        },
        None => {},
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is the end of line.
    if parser.next_token_is(Box::new(Tokens::EOL)) {
        // Get the next token.
        parser.next_token();
    }

    // Return the statement.
    Some(Box::new(Statements::VARIABLE(variable)))
  }
}
