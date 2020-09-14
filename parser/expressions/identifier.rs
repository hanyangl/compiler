use crate::tokens::Token;

use super::{
  Expression,
  Expressions,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
  this: Option<Token>,
  token: Token,
  value: String,
}

impl Expression for Identifier {
  fn new() -> Self {
    Self {
      this: None,
      token: Token::new_empty(),
      value: String::new(),
    }
  }

  fn from_token(token: Token) -> Self {
    Self {
      this: None,
      token: token.clone(),
      value: token.value.clone(),
    }
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    format!(
      "{}{}",
      match self.this.clone() {
        Some(this) => format!("{}.", this.value),
        None => String::new(),
      },
      self.value,
    )
  }
}

impl Identifier {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::IDENTIFIER(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::IDENTIFIER(Expression::from_token(token)))
  }

  pub fn get_value(&self) -> String {
    self.value.clone()
  }
}
