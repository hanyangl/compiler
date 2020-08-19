mod expression;
mod variable;

pub use expression::ExpressionStatement;
pub use variable::Variable;

use crate::tokens::Token;

pub trait Statement {
  /// Create an empty statement.
  fn new() -> Self;

  /// Create a statement with a token.
  fn from_token(token: Token) -> Self;

  /// Parse the statement to a string.
  fn string(self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statements {
  EXPRESSION(ExpressionStatement),
  VARIABLE(Variable),
}

impl Statements {
  pub fn get_expression(self) -> Option<ExpressionStatement> {
    match self {
      Statements::EXPRESSION(exp) => Some(exp),
      _ => None,
    }
  }
  
  pub fn get_variable(self) -> Option<Variable> {
    match self {
      Statements::VARIABLE(variable) => Some(variable),
      _ => None,
    }
  }
  
  pub fn string(self) -> String {
    match self {
      Statements::VARIABLE(variable) => variable.string(),
      Statements::EXPRESSION(exp) => exp.string(),
    }
  }
}
