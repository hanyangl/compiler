mod block;
mod export;
mod expression;
mod function;
mod import;
mod library;
mod return_s;
mod variable_set;
mod variable;

pub use block::Block;
pub use export::Export;
pub use expression::ExpressionStatement;
pub use function::Function;
pub use import::Import;
pub use library::Library;
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
  EXPORT(Export),
  EXPRESSION(ExpressionStatement),
  FUNCTION(Function),
  IMPORT(Import),
  LIBRARY(Library),
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

  pub fn is_block(self) -> bool {
    match self {
      Statements::BLOCK(_) => true,
      _ => false,
    }
  }

  pub fn get_export(self) -> Option<Export> {
    match self {
      Statements::EXPORT(export) => Some(export),
      _ => None,
    }
  }

  pub fn is_export(self) -> bool {
    match self {
      Statements::EXPORT(_) => true,
      _ => false,
    }
  }

  pub fn get_expression(self) -> Option<ExpressionStatement> {
    match self {
      Statements::EXPRESSION(exp) => Some(exp),
      _ => None,
    }
  }

  pub fn is_expression(self) -> bool {
    match self {
      Statements::EXPRESSION(_) => true,
      _ => false,
    }
  }

  pub fn get_function(self) -> Option<Function> {
    match self {
      Statements::FUNCTION(function) => Some(function),
      _ => None,
    }
  }

  pub fn is_function(self) -> bool {
    match self {
      Statements::FUNCTION(_) => true,
      _ => false,
    }
  }

  pub fn get_import(self) -> Option<Import> {
    match self {
      Statements::IMPORT(import) => Some(import),
      _ => None,
    }
  }

  pub fn is_import(self) -> bool {
    match self {
      Statements::IMPORT(_) => true,
      _ => false,
    }
  }

  pub fn get_library(self) -> Option<Library> {
    match self {
      Statements::LIBRARY(library) => Some(library),
      _ => None,
    }
  }

  pub fn is_library(self) -> bool {
    match self {
      Statements::LIBRARY(_) => true,
      _ => false,
    }
  }

  pub fn get_return(self) -> Option<Return> {
    match self {
      Statements::RETURN(return_s) => Some(return_s),
      _ => None,
    }
  }

  pub fn is_return(self) -> bool {
    match self {
      Statements::RETURN(_) => true,
      _ => false,
    }
  }

  pub fn get_variable_set(self) -> Option<VariableSet> {
    match self {
      Statements::VARIABLESET(variable_set) => Some(variable_set),
      _ => None,
    }
  }

  pub fn is_variable_set(self) -> bool {
    match self {
      Statements::VARIABLESET(_) => true,
      _ => false,
    }
  }

  pub fn get_variable(self) -> Option<Variable> {
    match self {
      Statements::VARIABLE(variable) => Some(variable),
      _ => None,
    }
  }

  pub fn is_variable(self) -> bool {
    match self {
      Statements::VARIABLE(_) => true,
      _ => false,
    }
  }

  pub fn string(self) -> String {
    match self {
      Statements::BLOCK(block) => block.string(),
      Statements::EXPORT(export) => export.string(),
      Statements::EXPRESSION(exp) => exp.string(),
      Statements::FUNCTION(function) => function.string(),
      Statements::IMPORT(import) => import.string(),
      Statements::LIBRARY(library) => library.string(),
      Statements::RETURN(return_s) => return_s.string(),
      Statements::VARIABLE(variable) => variable.string(),
      Statements::VARIABLESET(variable_set) => variable_set.string(),
    }
  }
}
