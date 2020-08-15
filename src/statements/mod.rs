pub mod expression;
pub mod variable;

use crate::data::Token;

pub trait Statement {
  /// Create an empty statement.
  fn new() -> Self;

  /// Create a statement with a token.
  fn from_token(token: &Token) -> Self;

  /// Parse the statement to a string.
  fn string(self) -> String;
}
