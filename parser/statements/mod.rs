mod block;
mod expression;
mod function;
mod return_s;
mod variable_set;
mod variable;

pub use block::Block;
pub use expression::ExpressionStatement;
pub use function::Function;
pub use return_s::Return;
pub use variable_set::VariableSet;
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
  BLOCK(Block),
  EXPRESSION(ExpressionStatement),
  FUNCTION(Function),
  RETURN(Return),
  VARIABLESET(VariableSet),
  VARIABLE(Variable),
}

impl Statements {
  pub fn get_block(self) -> Option<Block> {
    match self {
      Statements::BLOCK(block) => Some(block),
      _ => None,
    }
  }

  pub fn get_expression(self) -> Option<ExpressionStatement> {
    match self {
      Statements::EXPRESSION(exp) => Some(exp),
      _ => None,
    }
  }

  pub fn get_function(self) -> Option<Function> {
    match self {
      Statements::FUNCTION(function) => Some(function),
      _ => None,
    }
  }

  pub fn get_return(self) -> Option<Return> {
    match self {
      Statements::RETURN(return_s) => Some(return_s),
      _ => None,
    }
  }

  pub fn get_variable_set(self) -> Option<VariableSet> {
    match self {
      Statements::VARIABLESET(variable_set) => Some(variable_set),
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
      Statements::BLOCK(block) => block.string(),
      Statements::EXPRESSION(exp) => exp.string(),
      Statements::FUNCTION(function) => function.string(),
      Statements::RETURN(return_s) => return_s.string(),
      Statements::VARIABLE(variable) => variable.string(),
      Statements::VARIABLESET(variable_set) => variable_set.string(),
    }
  }
}
