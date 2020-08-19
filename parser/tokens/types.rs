use crate::expressions::Expressions;

use super::{TokenType, Tokens, Token};

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
  pub fn from_expression(exp: Box<Expressions>) -> Token {
    let mut token: Token = Token::new_empty();

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

    // Return token.
    token
  }
}
