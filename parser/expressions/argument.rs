use crate::{Environment, Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType, Tokens, Types};
use crate::types::expression_is_type;

use super::{
  Expressions,
  Expression,
  Call,
  parse as parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
  pub token: Token,
  pub data_type: Token,
  pub value: Option<Box<Expressions>>
}

impl Expression for Argument {
  fn new() -> Argument {
    Argument {
      token: Token::new_empty(),
      data_type: Token::new_empty(),
      value: None,
    }
  }

  fn from_token(token: Token) -> Argument {
    Argument {
      token,
      data_type: Token::new_empty(),
      value: None,
    }
  }

  fn string(self) -> String {
    let argument = format!(
      "{}: {}",
      self.token.value,
      self.data_type.value,
    );

    match self.value {
      Some(value) => format!("{} = {}", argument, value.string()),
      None => argument,
    }
  }
}

impl Argument {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::ARGUMENT(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::ARGUMENT(Expression::from_token(token)))
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Vec<Box<Expressions>>> {
    let mut arguments: Vec<Box<Expressions>> = Vec::new();

    // Check if the next token is a right parentheses.
    if parser.next_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    let mut has_default = false;
    while !parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Check if the next token is an identifier.
      if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
        let line = parser.get_error_line_next_token();
        parser.errors.push(format!("{} is not a valid identifier", line));
        return None;
      }

      let mut argument: Argument = Expression::from_token(parser.current_token.clone());

      // Check if the argument name is already in use.
      if environment.has_first_expression(argument.token.value.clone()) ||
        environment.has_first_statement(argument.token.value.clone()) {
        let line = parser.get_error_line_current_token();
        parser.errors.push(format!("{} `{}` is already a function argument.", line, argument.token.value));
        return None;
      }

      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        let line = parser.get_error_line_next_token();
        parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.next_token.value));
        return None;
      }

      // Get the next token.
      parser.next_token();

      // Parse argument data type.
      match parser.current_token.token.clone().get_type() {
        Some(_) => {
          // Set the argument data type.
          argument.data_type = parser.current_token.clone();
        },
        None => {
          let line = parser.get_error_line_current_token();
          parser.errors.push(format!("{} is not a valid data type.", line));
          return None;
        },
      }

      // Check if the next token is an assign sign.
      if parser.expect_token(Signs::new(Signs::ASSIGN)) {
        has_default = true;

        // Get the next token.
        parser.next_token();

        let current_token = parser.current_token.clone();

        // Parse default value expression.
        match parse_expression(parser, Some(argument.data_type.clone()), Precedence::LOWEST, environment, standard_library) {
          Some(value) => {
            let line = parser.get_error_line(
              current_token.line - 1,
              current_token.position - 1,
              value.clone().string().len(),
            );

            match argument.data_type.token.clone().get_type() {
              Some(data_type) => {
                // Check if the default value is the correct data type.
                if !expression_is_type(data_type, value.clone(), environment) {
                  parser.errors.push(format!("{} not satisfied the {} type.", line, argument.data_type.value));
                  return None;
                }
              },
              None => {},
            }

            // Set the default value.
            argument.value = Some(value);
          },
          None => {},
        }
      } else if has_default {
        let line = parser.get_error_line_next_token();
        parser.errors.push(format!("{} the argument must has a default value.", line));
        return None;
      }

      // Check if the next token is a comma.
      if parser.next_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();
      }

      // Check if the next token is a right parentheses.
      if parser.next_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
        // Get the next token.
        parser.next_token();
      }

      // Get the argument box expression.
      let argument_box = Box::new(Expressions::ARGUMENT(argument.clone()));

      // Set the argument to the enviroment.
      environment.set_expression(argument.token.value, argument_box.clone());

      // Add the argument to the list.
      arguments.push(argument_box);
    }

    // Return arguments.
    Some(arguments)
  }

  pub fn parse_call_arguments<'a>(
    parser: &'a mut Parser,
    call: Call,
    min_arguments: usize,
    max_arguments: usize,
    data_types: Vec<Token>,
    environment: &mut Environment,
  ) -> bool {
    let line = parser.get_error_line(call.token.line - 1, call.token.position - 1, call.token.value.len());

    // Check if the call has the minimum arguments.
    if call.arguments.clone().len() < min_arguments {
      parser.errors.push(
        format!(
          "{} expected {} minimum arguments, got {} instead.",
          line,
          min_arguments,
          call.arguments.len(),
        )
      );

      return false;
    }

    // Check if the call has the maximum arguments.
    if call.arguments.clone().len() > max_arguments {
      parser.errors.push(
        format!(
          "{} expected {} maximum arguments, got {} instead.",
          line,
          max_arguments,
          call.arguments.len(),
        )
      );

      return false;
    }

    let mut i: usize = 0;

    // Parse arguments data types.
    for argument in call.arguments.clone() {
      // Get the data type for the argument.
      let data_type_token = data_types[i].clone();

      // Get the data type token.
      match data_type_token.token.clone().get_type() {
        Some(data_type) => {
          // Get the data type of the argument.
          match Types::from_expression(argument.clone(), environment).token.get_type() {
            Some(data_type_argument) => {
              if data_type != data_type_argument {
                let line = parser.get_error_line(
                  argument.clone().token().line - 1,
                  argument.clone().token().position - 1,
                  argument.clone().string().len(),
                );

                parser.errors.push(format!("{} not satisfied the {} type.", line, data_type_token.value));

                return false;
              }
            },
            None => {},
          }
        },
        None => {},
      }

      i += 1;
    }

    true
  }
}
