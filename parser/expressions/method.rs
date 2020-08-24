use crate::{Environment, Parser};
use crate::tokens::Token;

use super::{
  Expressions,
  Expression,
  Identifier,
  HashMap,
  Call,
  AnonymousFunction,
  Argument,
  parse as parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
  pub token: Token,
  pub left: Option<Box<Expressions>>,
  pub data_type: Token,
  pub hashmap: Option<HashMap>,
  pub right: Option<Box<Expressions>>,
}

impl Expression for Method {
  fn new() -> Method {
    Method {
      token: Token::new_empty(),
      left: None,
      data_type: Token::from_value(String::from("void"), 0, 0),
      hashmap: None,
      right: None,
    }
  }

  fn from_token(token: Token) -> Method {
    let mut method: Method = Expression::new();

    method.token = token;

    method
  }

  fn string(self) -> String {
    format!(
      "{}{}{}",
      match self.left {
        Some(left) => left.string(),
        None => String::new(),
      },
      self.token.value,
      match self.right {
        Some(right) => right.string(),
        None => String::new(),
      },
    )
  }
}

impl Method {
  pub fn parse_hashmap<'a>(
    parser: &'a mut Parser,
    method: &mut Method,
    hashmap: HashMap,
    identifier: Identifier,
    environment: &mut Environment,
  ) -> bool {
    // Get the right identifier.
    match Identifier::get(method.right.clone()) {
      // Is an identifier.
      Some(right_identifier) => {
        let line = parser.get_error_line(right_identifier.token.line - 1, right_identifier.token.position - 1, right_identifier.value.len());

        // Check if the identifier exists in the hashmap.
        if !hashmap.clone().has_key(right_identifier.value.clone()) {
          parser.errors.push(format!("{} `{}` does not contain the `{}` key.", line, identifier.value, right_identifier.value));
          return false;
        }

        // Get the hashmap item.
        let hashmap_item = hashmap.clone().get_by_key(right_identifier.value.clone()).unwrap();

        // Check if the item is an anonymous function.
        if hashmap_item.value.clone().is_anonymous_function() {
          parser.errors.push(format!("{} `{}` is a function.", line, right_identifier.value));
          return false;
        }

        // Set the method data type.
        method.data_type = hashmap_item.data_type;

        if hashmap_item.value.clone().is_hashmap() {
          method.hashmap = hashmap_item.value.get_hashmap();
        }
      },

      // Is not an identifier.
      None => {
        // Get the right call.
        match Call::get(method.right.clone()) {
          // Is a call.
          Some(right_call) => {
            let line = parser.get_error_line(right_call.token.line - 1, right_call.token.position - 1, right_call.token.value.len());

            // Check if the identifier exists in the hashmap.
            if !hashmap.clone().has_key(right_call.token.value.clone()) {
              parser.errors.push(format!("{} `{}` does not contain the `{}` key.", line, identifier.value, right_call.token.value));
              return false;
            }

            // Get the hashmap item.
            let hashmap_item = hashmap.clone().get_by_key(right_call.token.value.clone()).unwrap();

            // Check if the item is not an anonymous function.
            if !hashmap_item.value.clone().is_anonymous_function() {
              parser.errors.push(format!("{} `{}` is not a function.", line, right_call.token.value));
              return false;
            }

            match AnonymousFunction::get_arguments(parser, hashmap_item.value.clone(), right_call.clone()) {
              Some((min_arguments, max_arguments, data_types, data_type)) => {
                method.data_type = data_type;

                if !Argument::parse_call_arguments(parser, right_call, min_arguments, max_arguments, data_types, environment) {
                  return false;
                }

                if hashmap_item.value.clone().is_hashmap() {
                  method.hashmap = hashmap_item.value.get_hashmap();
                }
              },
              None => {
                return false;
              },
            }
          },

          // Is not a call.
          None => {
            return false;
          },
        }
      },
    }

    true
  }

  pub fn parse_method<'a>(parser: &'a mut Parser, method: &mut Method, environment: &mut Environment) -> Option<Method> {
    match Method::get(method.left.clone()) {
      Some(left_method) => {
        match Identifier::get(left_method.right.clone()) {
          Some(left_identifier) => {
            match left_method.hashmap.clone() {
              Some(hashmap) => {
                if Method::parse_hashmap(parser, method, hashmap, left_identifier, environment) {
                  return Some(left_method.clone());
                }

                return None;
              },
              None => {},
            }
          },
          None => {},
        }
      },
      None => {},
    }

    None
  }

  pub fn parse_expressions<'a>(parser: &'a mut Parser, method: &mut Method, environment: &mut Environment) -> bool {
    match Identifier::get(method.left.clone()) {
      Some(left_identifier) => {
        method.hashmap = HashMap::get_from_environment(left_identifier.value.clone(), environment);

        match method.hashmap.clone() {
          Some(hashmap) => {
            if !Method::parse_hashmap(parser, method, hashmap, left_identifier, environment) {
              return false;
            }
          },
          None => {},
        }
      },
      None => {
        match Method::parse_method(parser, method, environment) {
          Some(_) => {},
          None => {
            return false;
          },
        }
      },
    }

    true
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    left_expression: Option<Box<Expressions>>,
    environment: &mut Environment,
  ) -> Option<Box<Expressions>> {
    let mut method: Method = Expression::from_token(parser.current_token.clone());

    // Set the left expression.
    method.left = left_expression;

    // Get the current precedence.
    let precedence = parser.current_precedence();

    // Get the next token.
    parser.next_token();

    // Set the right expression.
    method.right = parse_expression(parser, precedence, environment);

    // Parse expressions.
    if !Method::parse_expressions(parser, &mut method, environment) {
      return None;
    }

    // Return the method expression.
    Some(Box::new(Expressions::METHOD(method)))
  }

  pub fn get(expression: Option<Box<Expressions>>) -> Option<Method> {
    match expression {
      Some(exp) => exp.get_method(),
      None => None,
    }
  }
}
