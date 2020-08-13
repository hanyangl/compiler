mod variable;
pub mod expression;

pub use variable::Variable;
use crate::data::Token;

#[derive(Debug, Clone)]
pub struct Identifier {
  pub token: Token,
  pub value: String,
}

impl Identifier {
  /// Create a new empty identifier.
  pub fn new() -> Identifier {
    Identifier {
      token: Token::empty(),
      value: String::new(),
    }
  }

  /// Create a new identifier from a token.
  pub fn from_token(token: &Token) -> Identifier {
    Identifier {
      token: token.clone(),
      value: token.value.clone(),
    }
  }
}
