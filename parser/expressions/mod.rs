mod boolean;
mod identifier;
mod infix;
mod number;
mod parser;
mod string;

pub use boolean::Boolean;
pub use infix::Infix;
pub use identifier::Identifier;
pub use parser::parse;
pub use number::Number;
pub use string::StringE;

use crate::tokens::Token;

pub trait Expression {
  /// Create a new empty expression.
  fn new() -> Self;

  /// Create a new expression from a token.
  fn from_token(token: Token) -> Self;

  /// Get the expression value.
  fn string(self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expressions {
  BOOLEAN(Boolean),
  IDENTIFIER(Identifier),
  INFIX(Infix),
  NUMBER(Number),
  STRING(StringE),
}

impl Expressions {
  pub fn get_boolean(self) -> Option<Boolean> {
    match self {
      Expressions::BOOLEAN(boolean) => Some(boolean),
      _ => None,
    }
  }

  pub fn get_identifier(self) -> Option<Identifier> {
    match self {
      Expressions::IDENTIFIER(identifier) => Some(identifier),
      _ => None,
    }
  }

  pub fn get_infix(self) -> Option<Infix> {
    match self {
      Expressions::INFIX(infix) => Some(infix),
      _ => None,
    }
  }

  pub fn get_number(self) -> Option<Number> {
    match self {
      Expressions::NUMBER(number) => Some(number),
      _ => None,
    }
  }

  pub fn get_string(self) -> Option<StringE> {
    match self {
      Expressions::STRING(string) => Some(string),
      _ => None,
    }
  }

  pub fn token(self) -> Token {
    match self {
      Expressions::BOOLEAN(boolean) => boolean.token,
      Expressions::IDENTIFIER(identifier) => identifier.token,
      Expressions::INFIX(infix) => infix.token,
      Expressions::NUMBER(number) => number.token,
      Expressions::STRING(string) => string.token,
    }
  }

  pub fn string(self) -> String {
    match self {
      Expressions::BOOLEAN(boolean) => boolean.string(),
      Expressions::IDENTIFIER(identifier) => identifier.string(),
      Expressions::INFIX(infix) => infix.string(),
      Expressions::NUMBER(number) => number.string(),
      Expressions::STRING(string) => string.string(),
    }
  }
}
