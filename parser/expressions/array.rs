use crate::{Environment, Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType, Types};
use crate::types::expression_is_type;

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

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayIndex {
  pub token: Token,
  pub data_type: Token,
  pub index: Option<Box<Expressions>>,
}

impl Expression for ArrayIndex {
  fn new() -> ArrayIndex {
    ArrayIndex {
      token: Token::new_empty(),
      data_type: Token::new_empty(),
      index: None,
    }
  }

  fn from_token(token: Token) -> ArrayIndex {
    let mut array_index: ArrayIndex = Expression::new();

    array_index.token = token;

    array_index
  }

  fn string(self) -> String {
    format!(
      "{}[{}]",
      self.token.value,
      match self.index {
        Some(index) => index.string(),
        None => String::new(),
      },
    )
  }
}

impl ArrayIndex {
  pub fn parse_index<'a>(
    parser: &'a mut Parser,
    index: Option<Box<Expressions>>,
    left_token: Token,
    environment: &mut Environment,
  ) -> bool {
    match index.clone() {
      Some(position) => {
        let line = parser.get_error_line(
          position.clone().token().line - 1,
          left_token.position.clone(),
          position.clone().string().len(),
        );

        // Check if the expression is a valid number.
        if !expression_is_type(Types::NUMBER, position.clone(), environment) {
          parser.errors.push(format!("{} is not a valid `number` type.", line));
          return false;
        }

        // Check if the expression has a dot.
        if position.clone().string().contains('.') {
          parser.errors.push(format!("{} the index value can not contains a dot.", line));
          return false;
        }

        // Get the prefix expression.
        match position.clone().get_prefix() {
          Some(prefix) => {
            match prefix.clone().right {
              Some(right_exp) => {
                if right_exp.get_number().unwrap().value != 1.0 {
                  parser.errors.push(format!("{} the index can not be other than '-1' or a positive number.", line));
                  return false;
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

    true
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Expressions>> {
    let mut array_index: ArrayIndex = Expression::from_token(parser.current_token.clone());

    // Check if the next token is a left bracket.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACKET)) {
      let line = parser.get_error_line_next_token();
      parser.errors.push(format!("{} expect `[`, got `{}` instead.", line, parser.next_token.value));
      return None;
    }

    let left_token: Token = parser.current_token.clone();

    // Get the next token.
    parser.next_token();

    // Parse expression.
    array_index.index = parse_expression(parser, None, Precedence::LOWEST, environment, standard_library);

    // Parse array index.
    if !ArrayIndex::parse_index(parser, array_index.index.clone(), left_token, environment) {
      return None;
    }

    // Check if the next token is a right bracket.
    if !parser.expect_token(Signs::new(Signs::RIGHTBRACKET)) {
      let line = parser.get_error_line_next_token();
      parser.errors.push(format!("{} expect `]`, got `{}` instead.", line, parser.next_token.value));
      return None;
    }

    let token_line = parser.get_error_line(
      array_index.token.line - 1,
      array_index.token.position - 1,
      array_index.token.value.len()
    );

    // Get the environment expression for the identifier.
    match environment.get_statement(array_index.token.value.clone()) {
      // Exists.
      Some(env_stmt) => {
        // Check if the statement is a variable.
        match env_stmt.get_variable() {
          // Is a variable.
          Some(var) => {
            // Check if setting an array.
            match array_index.index.clone() {
              Some(position) => {
                if !var.data_type.token.clone().is_type() || !var.data_type.token.clone().get_type().unwrap().is_array() {
                  parser.errors.push(format!("{} `{}` is not an array.", token_line, array_index.token.value));
                  return None;
                }

                // Get the variable value.
                match var.value {
                  Some(last_value) => {
                    // Get the array from value.
                    match last_value.get_array() {
                      Some(array) => {
                        let mut index: usize = 0;

                        // Check if the position is a prefix expression. (-1)
                        if position.clone().is_prefix() {
                          index = array.types.len() - 1;
                        }
                        // Check if the position is a number expression.
                        else if position.clone().is_number() {
                          index = position.clone().get_number().unwrap().token.value.parse().expect("");
                        }

                        if array.types.len() - 1 < index {
                          let line = parser.get_error_line(
                            position.clone().token().line - 1,
                            position.clone().token().position - 1,
                            position.clone().string().len(),
                          );

                          parser.errors.push(format!("{} the len is {} but the index is {}.", line, array.types.len(), index));

                          return None;
                        }

                        // Check if the position array type is a number.
                        array_index.data_type = array.types[index].clone();
                      },
                      None => {}
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
            parser.errors.push(format!("{} `{}` is not a variable.", token_line, array_index.token.value));
            return None;
          }
        }
      },

      // Not exists.
      None => {
        parser.errors.push(format!("{} `{}` not found.", token_line, array_index.token.value));
        return None;
      },
    }

    // Return the box expression.
    Some(Box::new(Expressions::ARRAYINDEX(array_index)))
  }
}
