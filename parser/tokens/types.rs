use crate::Environment;
use crate::expressions::{Expressions, ArrayType};
use crate::statements::Statements;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
  // Basic
  NULL,
  UNDEFINED,
  STRING,
  NUMBER,
  BOOLEAN,

  // Objects
  HASHMAP,
  ARRAY(ArrayType),

  // Function
  VOID,
}

impl TokenType for Types {
  fn new(data_type: Types) -> Box<Tokens> {
    Box::new(Tokens::TYPE(data_type))
  }

  fn from_value(value: String) -> Option<Box<Tokens>> {
    // Parse array type.
    if value.clone().ends_with("[]") {
      return Some(TokenType::new(Types::ARRAY(ArrayType::parse(value))));
    }

    match value.as_str() {
      // Basic
      "null" => Some(TokenType::new(Types::NULL)),
      "undefined" => Some(TokenType::new(Types::UNDEFINED)),
      "string" => Some(TokenType::new(Types::STRING)),
      "number" => Some(TokenType::new(Types::NUMBER)),
      "boolean" => Some(TokenType::new(Types::BOOLEAN)),

      // Objects
      "hashmap" => Some(TokenType::new(Types::HASHMAP)),

      // Function
      "void" => Some(TokenType::new(Types::VOID)),

      // Default
      _ => None,
    }
  }
}

impl Types {
  pub fn is_array(self) -> bool {
    match self {
      Types::ARRAY(_) => true,
      _ => false,
    }
  }

  pub fn get_array(self) -> Option<ArrayType> {
    match self {
      Types::ARRAY(array) => Some(array),
      _ => None,
    }
  }

  pub fn from_statement(statement: Box<Statements>) -> Token {
    let mut token: Token = Token::new_empty();

    // Parse variable.
    match statement.clone().get_variable() {
      Some(variable) => {
        token = variable.data_type;
      },
      None => {},
    }

    // Parse function.
    match statement.clone().get_function() {
      Some(function) => {
        token = function.data_type;
      },
      None => {},
    }

    // Return token.
    token
  }

  pub fn from_expression(exp: Box<Expressions>, environment: &mut Environment) -> Token {
    let mut token: Token = Token::new_empty();

    // Parse identifier.
    match exp.clone().get_identifier() {
      Some(identifier) => {
        // Get expression.
        match environment.get_expression(identifier.value.clone()) {
          Some(env_exp) => {
            token = Types::from_expression(env_exp, environment);
          },
          None => {},
        }

        // Get statement.
        match environment.get_statement(identifier.value) {
          Some(env_stmt) => {
            token = Types::from_statement(env_stmt);
          },
          None => {},
        }
      },
      None => {},
    }

    // Parse string.
    match exp.clone().get_string() {
      Some(_) => {
        token = Token::from_value(String::from("string"), 0, 0);
      },
      None => {},
    }

    // Parse number.
    match exp.clone().get_number() {
      Some(_) => {
        token = Token::from_value(String::from("number"), 0, 0);
      },
      None => {},
    }

    // Parse boolean.
    match exp.clone().get_boolean() {
      Some(_) => {
        token = Token::from_value(String::from("boolean"), 0, 0);
      },
      None => {},
    }

    // Parse infix.
    match exp.clone().get_infix() {
      Some(infix) => {
        let operator = infix.token.token;

        // Get the left expression.
        let left = match infix.left {
          Some(left) => Types::from_expression(left, environment),
          None => Token::new_empty(),
        };

        // Get the right expression.
        let right = match infix.right {
          Some(right) => Types::from_expression(right, environment),
          None => Token::new_empty(),
        };

        // Parse to string.
        if operator.clone().expect_sign(Signs::PLUS) && (
          left.token.clone().expect_type(Types::STRING) ||
          right.token.clone().expect_type(Types::STRING)
        ) {
          token = Token::from_value(String::from("string"), 0, 0);
        }

        // Parse to number.
        else if left.token.clone().expect_type(Types::NUMBER) &&
          right.token.clone().expect_type(Types::NUMBER) && (
          operator.clone().expect_sign(Signs::PLUS) ||
          operator.clone().expect_sign(Signs::MINUS) ||
          operator.clone().expect_sign(Signs::DIVIDE) ||
          operator.clone().expect_sign(Signs::MULTIPLY) ||
          operator.clone().expect_sign(Signs::EMPOWERMENT) ||
          operator.clone().expect_sign(Signs::MODULE)
        ) {
          token = Token::from_value(String::from("number"), 0, 0);
        }

        // Parse to boolean.
        else if operator.clone().expect_sign(Signs::EQUAL) ||
          operator.clone().expect_sign(Signs::EQUALTYPE) ||
          operator.clone().expect_sign(Signs::NOTEQUAL) ||
          operator.clone().expect_sign(Signs::NOTEQUALTYPE) ||
          operator.clone().expect_sign(Signs::LESSTHAN) ||
          operator.clone().expect_sign(Signs::LESSOREQUALTHAN) ||
          operator.clone().expect_sign(Signs::GREATERTHAN) ||
          operator.clone().expect_sign(Signs::GREATEROREQUALTHAN) {
          token = Token::from_value(String::from("boolean"), 0, 0);
        }
      },
      None => {},
    }

    // Parse prefix.
    match exp.clone().get_prefix() {
      Some(prefix) => {
        let operator = prefix.token.token.clone();

        // Parse negation prefix.
        if operator.clone().expect_sign(Signs::NEGATION) {
          token = Token::from_value(String::from("boolean"), 0, 0);
        }

        // Parse minus prefix.
        if operator.clone().expect_sign(Signs::MINUS) {
          token = Token::from_value(String::from("number"), 0, 0);
        }
      },
      None => {},
    }

    // Parse argument.
    match exp.clone().get_argument() {
      Some(argument) => {
        token = argument.data_type;
      },
      None => {},
    }

    // Parse anonymous function.
    match exp.clone().get_anonymous_function() {
      Some(_) => {
        token = Token::from_value(String::from("void"), 0, 0);
      },
      None => {},
    }

    // Parse call.
    match exp.clone().get_call() {
      Some(call) => {
        token = call.data_type.clone();
      },
      None => {},
    }

    // Parse hashmap.
    match exp.clone().get_hashmap() {
      Some(_) => {
        token = Token::from_value(String::from("hashmap"), 0, 0);
      },
      None => {},
    }

    // Parse method.
    match exp.clone().get_method() {
      Some(method) => {
        token = method.data_type.clone();
      },
      None => {},
    }

    // Parse array.
    match exp.clone().get_array() {
      Some(array) => {
        let mut types: Vec<String> = Vec::new();

        for data_type in array.types {
          let mut has = false;

          for type_string in types.iter() {
            if type_string.clone() == data_type.value.clone() {
              has = true;
              break;
            }
          }

          if has {
            continue;
          }

          types.push(data_type.value);
        }

        token = Token::from_value(format!("{}[]", types.join(" | ")), 0, 0);
      },
      None => {},
    }

    // Parse array index.
    match exp.clone().get_array_index() {
      Some(array_index) => {
        token = array_index.data_type;
      },
      None => {},
    }

    // Return token.
    token
  }
}
