use crate::Environment;
use crate::expressions::{Expressions, Number, parse as parse_expression};
use crate::{Parser, Precedence};
use crate::tokens::*;
use crate::types::expression_is_type;

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableSet {
  pub token: Token,
  pub array_position: Option<Box<Expressions>>,
  pub assign: Token,
  pub value: Option<Box<Expressions>>,
}

impl Statement for VariableSet {
  fn new() -> VariableSet {
    VariableSet {
      token: Token::new_empty(),
      array_position: None,
      assign: Token::new_empty(),
      value: None,
    }
  }

  fn from_token(token: Token) -> VariableSet {
    let mut variable: VariableSet = Statement::new();

    variable.token = token;

    variable
  }

  fn string(self) -> String {
    if self.assign.token.clone().expect_sign(Signs::PLUSPLUS) ||
      self.assign.token.clone().expect_sign(Signs::MINUSMINUS) {
      return format!("{}{};", self.token.value, self.assign.value);
    }

    format!(
      "{} {} {};",
      format!(
        "{}{}",
        self.token.value,
        match self.array_position {
          Some(position) => format!("[{}]", position.string()),
          None => String::new(),
        },
      ),
      self.assign.value,
      match self.value {
        Some(value) => value.string(),
        None => String::new(),
      },
    )
  }
}

impl VariableSet {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Statements>> {
    let mut variable: VariableSet = Statement::from_token(parser.current_token.clone());

    // Parse (identifier)[position]
    if parser.expect_token(Signs::new(Signs::LEFTBRACKET)) {
      let left_token: Token = parser.current_token.clone();

      // Get the next token.
      parser.next_token();

      // Parse expression.
      variable.array_position = parse_expression(parser, None, Precedence::LOWEST, environment, standard_library);

      match variable.array_position.clone() {
        Some(position) => {
          let line = parser.get_error_line(
            position.clone().token().line - 1,
            left_token.position.clone(),
            position.clone().string().len(),
          );

          // Check if the expression is a valid number.
          if !expression_is_type(Types::NUMBER, position.clone(), environment) {
            parser.errors.push(format!("{} is not a valid `number` type.", line));
            return None;
          }

          // Check if the expression has a dot.
          if position.clone().string().contains('.') {
            parser.errors.push(format!("{} the index value can not contains a dot.", line));
            return None;
          }

          // Get the prefix expression.
          match position.clone().get_prefix() {
            Some(prefix) => {
              match prefix.clone().right {
                Some(right_exp) => {
                  if right_exp.get_number().unwrap().value != 1.0 {
                    parser.errors.push(format!("{} the index can not be other than '-1' or a positive number.", line));
                    return None;
                  }
                },
                None => {},
              }
            },
            None => {},
          }
        },
        None => {},
      }

      // Check if the next token is a right bracket.
      if !parser.expect_token(Signs::new(Signs::RIGHTBRACKET)) {
        let line = parser.get_error_line_next_token();
        parser.errors.push(format!("{} expect `]`, got `{}` instead.", line, parser.next_token.value));
        return None;
      }
    }

    // Parse assigns signs.
    if parser.expect_token(Signs::new(Signs::ASSIGN)) ||
      parser.expect_token(Signs::new(Signs::PLUSASSIGN)) ||
      parser.expect_token(Signs::new(Signs::MINUSASSIGN)) ||
      parser.expect_token(Signs::new(Signs::MULTIPLYASSIGN)) ||
      parser.expect_token(Signs::new(Signs::DIVIDEASSIGN)) {
      // Set the variable assign token.
      variable.assign = parser.current_token.clone();

      // Get the next token.
      parser.next_token();

      // Parse the value expression.
      variable.value = parse_expression(parser, None, Precedence::LOWEST, environment, standard_library);
    }

    // Parse ++ and -- signs.
    else if parser.expect_token(Signs::new(Signs::PLUSPLUS)) ||
      parser.expect_token(Signs::new(Signs::MINUSMINUS)) {
      // Set the variable assign token.
      variable.assign = parser.current_token.clone();

      // Get the next token.
      parser.next_token();

      // Set a one value.
      variable.value = Some(
        Number::new_box_from_token(
          Token::new(
            Box::new(Tokens::NUMBER),
            String::from("1"),
            0,
            0,
          )
        )
      );
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    // Get the environment expression for the variable name.
    match environment.get_statement(variable.token.value.clone()) {
      Some(env_stmt) => {
        // Check if the statement is a variable.
        match env_stmt.clone().get_variable() {
          // Is a variable.
          Some(var) => {
            let line = parser.get_error_line(
              variable.token.line.clone() - 1,
              variable.token.position.clone() - 1,
              variable.token.value.clone().len()
            );

            // Check if the original variable is a const.
            if var.token.token.clone().expect_keyword(Keywords::CONST) {
              parser.errors.push(format!("{} you can not set a value to a const.", line));
              return None;
            }

            // Check if setting an array.
            match variable.array_position.clone() {
              Some(_) => {
                if !var.data_type.token.clone().is_type() || !var.data_type.token.clone().get_type().unwrap().is_array() {
                  parser.errors.push(format!("{} `{}` is not an array.", line, variable.token.value));
                  return None;
                }
              },
              None => {},
            }

            if !variable.assign.token.clone().expect_sign(Signs::ASSIGN) &&
              !var.data_type.token.clone().expect_type(Types::NUMBER) {
              let line = parser.get_error_line(
                variable.token.line - 1,
                variable.token.position - 1,
                variable.token.value.len()
              );

              parser.errors.push(format!("{} the variable is not of `number` type.", line));

              return None;
            }

            // Parse new value.
            match variable.value.clone() {
              Some(new_value) => {
                match var.data_type.token.clone().get_type() {
                  Some(data_type) => {
                    if !expression_is_type(data_type.clone(), new_value.clone(), environment) {
                      let line = parser.get_error_line(
                        new_value.clone().token().line - 1,
                        new_value.clone().token().position - 1,
                        new_value.clone().string().len(),
                      );

                      parser.errors.push(format!("{} not satisfied the `{}` type.", line, var.data_type.value));

                      return None;
                    }
                  },
                  None => {},
                }
              },
              None => {},
            }
          },

          // Is not a variable.
          None => {
            let line = parser.get_error_line(
              variable.token.line - 1,
              variable.token.position - 1,
              variable.token.value.len()
            );

            parser.errors.push(format!("{} `{}` is not a variable.", line, variable.token.value));

            return None;
          },
        }
      },
      None => {
        let line = parser.get_error_line(
          variable.token.line - 1,
          variable.token.position - 1,
          variable.token.value.len()
        );

        parser.errors.push(format!("{} `{}` not found.", line, variable.token.value));

        return None;
      },
    }

    // Return variable set statement.
    Some(Box::new(Statements::VARIABLESET(variable)))
  }
}
