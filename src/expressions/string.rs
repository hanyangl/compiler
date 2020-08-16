use crate::data::Token;

use super::Expression;

#[derive(Debug, Clone, PartialEq)]
pub struct StringE {
  pub token: Token,
  pub value: String,
}

impl Expression for StringE {
  fn new() -> StringE {
    StringE {
      token: Token::empty(),
      value: String::new(),
    }
  }

  fn from_token(token: &Token) -> StringE {
    StringE {
      token: token.clone(),
      value: token.value.clone(),
    }
  }

  fn string(self) -> String {
    self.value
  }
}