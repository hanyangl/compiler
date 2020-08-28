use crate::{Environment, Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType, Types};

use super::{Expressions, Expression, parse as parse_expression};

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
  pub types: Vec<Token>,
  pub value: String,
}

impl ArrayType {
  pub fn new() -> ArrayType {
    ArrayType {
      types: Vec::new(),
      value: String::new(),
    }
  }

  pub fn parse(value: String) -> ArrayType {
    let mut array_type = ArrayType::new();

    array_type.value = value.clone();

    let new_value = &value[0..value.len() - 2];
    let new_token = Token::from_value(new_value.to_string(), 0, 0);

    if new_token.token.clone().is_type() {
      array_type.types.push(new_token.clone());
    }

    array_type
  }

  pub fn string(self) -> String {
    self.value
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
  pub token: Token,
  pub types: Vec<Token>,
  pub data: Vec<Box<Expressions>>,
}

impl Expression for Array {
  fn new() -> Array {
    Array {
      token: Token::new_empty(),
      types: Vec::new(),
      data: Vec::new(),
    }
  }

  fn from_token(token: Token) -> Array {
    let mut array: Array = Expression::new();

    array.token = token;

    array
  }

  fn string(self) -> String {
    let mut data: Vec<String> = Vec::new();

    for expression in self.data {
      data.push(expression.string());
    }

    format!("[{}]", data.join(", "))
  }
}

impl Array {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    data_type: Option<Token>,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Expressions>> {
    let mut array: Array = Expression::from_token(parser.current_token.clone());

    // Get the next token.
    parser.next_token();

    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACKET)) {
      // Parse expression.
      match parse_expression(parser, None, Precedence::LOWEST, environment, standard_library) {
        Some(expression) => {
          let exp_type = Types::from_expression(expression.clone(), environment);

          match data_type.clone() {
            Some(data_type_token) => {
              let mut has_type = false;
              let mut array_types = String::new();

              // Get the token type.
              match data_type_token.token.clone().get_type() {
                Some(data_type_type) => {
                  // Get the array type token.
                  match data_type_type.clone().get_array() {
                    Some(array) => {
                      has_type = array.types.contains(&exp_type);
                      array_types = array.string();
                    },
                    None => {},
                  }
                },
                None => {},
              }

              if !has_type {
                let line = parser.get_error_line_current_token();

                parser.errors.push(format!(
                  "{} `{}` not satisfied the `{}` type.",
                  line,
                  parser.current_token.value,
                  array_types,
                ));

                return None;
              }
            },
            None => {},
          }

          array.types.push(exp_type);
          array.data.push(expression);
        },
        None => {
          println!("TODO(array): Parse expression");
          return None;
        },
      }

      // Check if the next token is a comma.
      if parser.next_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is a right bracket.
    if parser.current_token_is(Signs::new(Signs::RIGHTBRACKET)) {
      // Get the next token.
      parser.next_token();
    }

    // Return the array expression.
    Some(Box::new(Expressions::ARRAY(array)))
  }
}
