pub mod boolean;
pub mod infix;
pub mod integer;
pub mod prefix;

use crate::data::Token;

#[derive(Debug, Clone)]
pub struct Identifier {
  token: Token,
  value: String,
}

pub trait Expression {
  /// Create a new empty expression.
  fn new() -> Self;

  /// Create a new expression from a token.
  fn from_token(token: &Token) -> Self;

  /// Get the expression value.
  fn string(self) -> String;
}

impl Expression for Identifier {
  fn new() -> Identifier {
    Identifier {
      token: Token::empty(),
      value: String::new(),
    }
  }

  fn from_token(token: &Token) -> Identifier {
    Identifier {
      token: token.clone(),
      value: token.value.clone(),
    }
  }

  fn string(self) -> String {
    self.value
  }
}
