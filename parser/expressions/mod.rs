mod identifier;
mod number;
mod parser;
mod string;

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
  IDENTIFIER(Identifier),
  NUMBER(Number),
  STRING(StringE),
}

impl Expressions {
  pub fn get_identifier(self) -> Option<Identifier> {
    match self {
      Expressions::IDENTIFIER(identifier) => Some(identifier),
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

  pub fn string(self) -> String {
    match self {
      Expressions::IDENTIFIER(identifier) => identifier.string(),
      Expressions::NUMBER(number) => number.string(),
      Expressions::STRING(string) => string.string(),
    }
  }
}
