use crate::{Environment, Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType};

use super::{
  Expressions,
  Expression,
  AnonymousFunction,
  Argument,
  parse as parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Call {
  pub token: Token,
  pub data_type: Token,
  pub arguments: Vec<Box<Expressions>>,
  pub semicolon: Option<Token>,
}

impl Expression for Call {
  fn new() -> Call {
    Call {
      token: Token::new_empty(),
      data_type: Token::from_value(String::from("void"), 0, 0),
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
  pub fn get_arguments<'a>(
    parser: &'a mut Parser,
    call: Call,
    environment: &mut Environment,
  ) -> Option<(usize, usize, Vec<Token>, Token)> {
    let mut min_arguments: usize = 0;
    let mut max_arguments: usize = 0;
    let mut data_types: Vec<Token> = Vec::new();

    // Get the statement from the environment.
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

            return Some((min_arguments, max_arguments, data_types, function.data_type));
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
                    return AnonymousFunction::get_arguments(parser, value, call.clone());
                  },

                  // Without value.
                  None => {
                    println!("TODO(call): Without value");
                    return None;
                  },
                }
              },

              // Is not a variable.
              None => {
                println!("TODO(call): Is not a variable");
                return None;
              },
            }
          },
        }
      },

      // Default
      None => None,
    }
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Expressions>> {
    let mut call: Call = Expression::from_token(parser.current_token.clone());

    // Check if the call identifier exists.
    if !environment.has_expression(call.token.value.clone()) &&
      !environment.has_statement(call.token.value.clone()) &&
      !parser.last_token_is(Signs::new(Signs::ARROW)) {
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
      match parse_expression(parser, None, Precedence::LOWEST, environment, standard_library) {
        Some(argument) => {
          call.arguments.push(argument);
        },
        None => {},
      }

      // Get the next token.
      parser.next_token();
    }

    // Get statement from environment.
    match Call::get_arguments(parser, call.clone(), environment) {
      Some((min_arguments, max_arguments, data_types, data_type)) => {
        call.data_type = data_type;

        if !Argument::parse_call_arguments(parser, call.clone(), min_arguments, max_arguments, data_types, environment) {
          return None;
        }
      },
      None => {},
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

    // Return the call expression.
    Some(Box::new(Expressions::CALL(call)))
  }

  pub fn get(expression: Option<Box<Expressions>>) -> Option<Call> {
    match expression {
      Some(exp) => exp.get_call(),
      None => None,
    }
  }
}
