use crate::{Environment, Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType, Types, Tokens};

use super::{Expressions, Expression, parse as parse_expression};

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
  pub token: Token,
  pub arguments: Vec<Box<Expressions>>,
  pub semicolon: Option<Token>,
}

impl Expression for Call {
  fn new() -> Call {
    Call {
      token: Token::new_empty(),
      arguments: Vec::new(),
      semicolon: None,
    }
  }

  fn from_token(token: Token) -> Call {
    let mut call: Call = Expression::new();

    call.token = token;

    call
  }

  fn string(self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.arguments {
      arguments.push(argument.string());
    }

    format!(
      "{}({}){}",
      self.token.value,
      arguments.join(", "),
      match self.semicolon {
        Some(semicolon) => semicolon.value,
        None => String::new(),
      },
    )
  }
}

impl Call {
  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Expressions>> {
    let mut call: Call = Expression::from_token(parser.current_token.clone());

    // Check if the call identifier exists.
    if !environment.has_expression(call.token.value.clone()) &&
      !environment.has_statement(call.token.value.clone()) {
      let line = parser.get_error_line_current_token();

      parser.errors.push(format!("{} identifier not found.", line));

      return None;
    }

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      let line = parser.get_error_line_next_token();

      parser.errors.push(format!("{} expect `(`, got `{}` instead.", line, parser.next_token.value));

      return None;
    }

    // Get the next token.
    parser.next_token();

    // Get all arguments.
    while !parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Check if the current token is a comma.
      if parser.current_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();
      }

      // Parse expression.
      match parse_expression(parser, Precedence::LOWEST, environment) {
        Some(argument) => {
          call.arguments.push(argument);
        },
        None => {},
      }

      // Get the next token.
      parser.next_token();
    }

    let line = parser.get_error_line(call.token.line - 1, call.token.position - 1, call.token.value.len());
    let mut min_arguments: usize = 0;
    let mut max_arguments: usize = 0;
    let mut data_types: Vec<Token> = Vec::new();

    // Get statement from environment.
    match environment.get_statement(call.token.value.clone()) {
      Some(statement) => {
        // Get function statement.
        match statement.clone().get_function() {
          // Is a function.
          Some(function) => {
            for argument_exp in function.arguments.clone() {
              // Get argument expression.
              match argument_exp.get_argument() {
                Some(argument) => {
                  // Add argument data type to the data types list.
                  data_types.push(argument.data_type);

                  // Check if the argument has a default value.
                  match argument.value {
                    // With default value.
                    Some(_) => {
                      max_arguments += 1;
                    },
                    // Without default value.
                    None => {
                      min_arguments += 1;
                      max_arguments += 1;
                    },
                  }
                },
                None => {},
              }
            }
          },

          // Is not a function.
          None => {
            // Check if the statement is a variable.
            match statement.clone().get_variable() {
              // Is a variable.
              Some(variable) => {
                // Get the variable value.
                match variable.value {
                  // With value.
                  Some(value) => {
                    // Get the anonymous function.
                    match value.get_anonymous_function() {
                      // Is an anonymous function.
                      Some(anonymous_function) => {
                        for argument_exp in anonymous_function.arguments.clone() {
                          // Get argument expression.
                          match argument_exp.get_argument() {
                            Some(argument) => {
                              // Add argument data type to the data types list.
                              data_types.push(argument.data_type);

                              // Check if the argument has a default value.
                              match argument.value {
                                // With default value.
                                Some(_) => {
                                  max_arguments += 1;
                                },
                                // Without default value.
                                None => {
                                  min_arguments += 1;
                                  max_arguments += 1;
                                },
                              }
                            },
                            None => {},
                          }
                        }
                      },

                      // Is not an anonymous function.
                      None => {
                        parser.errors.push(format!("{} the identifier is not a function.", line));
                        return None;
                      },
                    }
                  },

                  // Without value.
                  None => {
                    parser.errors.push(format!("{} the variable is not initialized.", line));
                    return None;
                  },
                }
              },

              // Is not a variable.
              None => {
                parser.errors.push(format!("{} the identifier is not a function.", line));
                return None;
              },
            }
          },
        }
      },

      // Default
      None => {},
    }

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

      return None;
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

      return None;
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

                return None;
              }
            },
            None => {},
          }
        },
        None => {},
      }

      i += 1;
    }

    // Check if the current token is a right parentheses.
    if parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is a semicolon.
    if parser.current_token_is(Signs::new(Signs::SEMICOLON)) {
      call.semicolon = Some(parser.current_token.clone());

      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is the end of line.
    if parser.current_token_is(Box::new(Tokens::EOL)) {
      // Get the next token.
      parser.next_token();
    }

    // Return the call expression.
    Some(Box::new(Expressions::CALL(call)))
  }
}
