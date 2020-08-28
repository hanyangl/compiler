use crate::Environment;
use crate::expressions::{ArrayIndex, Expressions, Number, parse as parse_expression};
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

      // Parse array position.
      if !ArrayIndex::parse_index(parser, variable.array_position.clone(), left_token, environment) {
        return None;
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

    let token_line = parser.get_error_line(
      variable.token.line - 1,
      variable.token.position - 1,
      variable.token.value.len()
    );

    // Get the environment expression for the variable name.
    match environment.get_statement(variable.token.value.clone()) {
      Some(env_stmt) => {
        // Check if the statement is a variable.
        match env_stmt.clone().get_variable() {
          // Is a variable.
          Some(var) => {
            // Check if the original variable is a const.
            if var.token.token.clone().expect_keyword(Keywords::CONST) {
              parser.errors.push(format!("{} you can not set a value to a const.", token_line));
              return None;
            }

            // Check if setting an array.
            match variable.array_position.clone() {
              Some(_) => {
                if !var.data_type.token.clone().is_type() || !var.data_type.token.clone().get_type().unwrap().is_array() {
                  parser.errors.push(format!("{} `{}` is not an array.", token_line, variable.token.value));
                  return None;
                }
              },
              None => {},
            }

            // Check if it is not an assign token.
            if !variable.assign.token.clone().expect_sign(Signs::ASSIGN) {
              let mut is_valid = true;

              // Check if the value token is a data type.
              if var.data_type.token.clone().is_type() {
                // Check if the value data type is an array.
                if var.data_type.token.clone().get_type().unwrap().is_array() {
                  // Get the variable value.
                  match var.value {
                    Some(last_value) => {
                      // Get the array from value.
                      match last_value.get_array() {
                        Some(array) => {
                          // Get the array position.
                          match variable.array_position.clone() {
                            Some(position) => {
                              let mut index: usize = 0;

                              // Check if the position is a prefix expression. (-1)
                              if position.clone().is_prefix() {
                                index = array.types.len() - 1;
                              }
                              // Check if the position is a number expression.
                              else if position.clone().is_number() {
                                index = position.get_number().unwrap().token.value.parse().expect("");
                              }

                              // Check if the position array type is a number.
                              is_valid = array.types[index].token.clone().expect_type(Types::NUMBER);
                            },
                            None => {},
                          }
                        },
                        None => {}
                      }
                    },
                    None => {},
                  }
                }
                // Check if the value data type is a number.
                else if !var.data_type.token.clone().expect_type(Types::NUMBER) {
                  is_valid = false;
                }
              }

              if !is_valid {
                parser.errors.push(format!("{} `{}` is not of `number` type.", token_line, variable.token.value));
                return None;
              }
            }

            // Parse new value.
            match variable.value.clone() {
              Some(new_value) => {
                match var.data_type.token.clone().get_type() {
                  Some(data_type) => {
                    // Check if the value expression is a valid type.
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
            parser.errors.push(format!("{} `{}` is not a variable.", token_line, variable.token.value));
            return None;
          },
        }
      },
      None => {
        parser.errors.push(format!("{} `{}` not found.", token_line, variable.token.value));
        return None;
      },
    }

    // Return variable set statement.
    Some(Box::new(Statements::VARIABLESET(variable)))
  }
}
