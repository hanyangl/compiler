pub mod block;
pub mod expression;
pub mod return_s;
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

#[derive(Debug, Clone)]
pub enum Statements {
  EXPRESSION(expression::ExpressionStatement),
  RETURN(return_s::Return),
  VARIABLE(variable::Variable),
}

impl Statements {
  /// Get the expression statement.
  pub fn get_expression(self) -> Option<expression::ExpressionStatement> {
    match self {
      Statements::EXPRESSION(exp) => Some(exp),
      _ => None,
    }
  }

  /// Get the return statement.
  pub fn get_return(self) -> Option<return_s::Return> {
    match self {
      Statements::RETURN(return_stmt) => Some(return_stmt),
      _ => None,
    }
  }

  /// Get the variable statement.
  pub fn get_variable(self) -> Option<variable::Variable> {
    match self {
      Statements::VARIABLE(variable) => Some(variable),
      _ => None,
    }
  }

  /// Get the statement as string.
  pub fn string(self) -> String {
    match self {
      Statements::EXPRESSION(exp) => exp.string(),
      Statements::RETURN(return_stmt) => return_stmt.string(),
      Statements::VARIABLE(variable) => variable.string(),
    }
  }
}
