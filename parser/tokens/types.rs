use crate::Environment;
use crate::expressions::Expressions;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
  // Basic
  NULL,
  UNDEFINED,
  STRING,
  NUMBER,
  BOOLEAN,

  // Function
  VOID,
}

impl TokenType for Types {
  fn new(data_type: Types) -> Box<Tokens> {
    Box::new(Tokens::TYPE(data_type))
  }

  fn from_value(value: String) -> Option<Box<Tokens>> {
    match value.as_str() {
      // Basic
      "null" => Some(TokenType::new(Types::NULL)),
      "undefined" => Some(TokenType::new(Types::UNDEFINED)),
      "string" => Some(TokenType::new(Types::STRING)),
      "number" => Some(TokenType::new(Types::NUMBER)),
      "boolean" => Some(TokenType::new(Types::BOOLEAN)),

      // Function
      "void" => Some(TokenType::new(Types::VOID)),

      // Default
      _ => None,
    }
  }
}

impl Types {
  pub fn from_expression(exp: Box<Expressions>, environment: &mut Environment) -> Token {
    let mut token: Token = Token::new_empty();

    // Parse identifier.
    match exp.clone().get_identifier() {
      Some(identifier) => {
        match environment.get_expression(identifier.value) {
          Some(env_exp) => {
            token = Types::from_expression(env_exp, environment);
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
        if operator.clone().is_sign(Signs::PLUS) && (
          left.token.clone().is_type(Types::STRING) ||
          right.token.clone().is_type(Types::STRING)
        ) {
          token = Token::from_value(String::from("string"), 0, 0);
        }

        // Parse to number.
        else if left.token.clone().is_type(Types::NUMBER) && right.token.clone().is_type(Types::NUMBER) && (
          operator.clone().is_sign(Signs::PLUS) ||
          operator.clone().is_sign(Signs::MINUS) ||
          operator.clone().is_sign(Signs::DIVIDE) ||
          operator.clone().is_sign(Signs::MULTIPLY) ||
          operator.clone().is_sign(Signs::EMPOWERMENT) ||
          operator.clone().is_sign(Signs::MODULE)
        ) {
          token = Token::from_value(String::from("number"), 0, 0);
        }

        // Parse to boolean.
        else if operator.clone().is_sign(Signs::EQUAL) ||
          operator.clone().is_sign(Signs::EQUALTYPE) ||
          operator.clone().is_sign(Signs::NOTEQUAL) ||
          operator.clone().is_sign(Signs::NOTEQUALTYPE) ||
          operator.clone().is_sign(Signs::LESSTHAN) ||
          operator.clone().is_sign(Signs::LESSOREQUALTHAN) ||
          operator.clone().is_sign(Signs::GREATERTHAN) ||
          operator.clone().is_sign(Signs::GREATEROREQUALTHAN) {
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
        if operator.clone().is_sign(Signs::NEGATION) {
          token = Token::from_value(String::from("boolean"), 0, 0);
        }

        // Parse minus prefix.
        if operator.clone().is_sign(Signs::MINUS) {
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

    // Return token.
    token
  }
}
