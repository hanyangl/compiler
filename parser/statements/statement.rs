use crate::tokens::Token;

use super::*;

pub trait Statement {
  /// Create an empty statement.
  fn new() -> Self;

  /// Create a statement with a token.
  fn from_token(token: Token) -> Self;

  /// Parse the statement to a string.
  fn string(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statements {
  BLOCK(Block),
  EXPORT(Export),
  EXPRESSION(ExpressionStatement),
  FUNCTION(Function),
  IFELSE(IfElse),
  IMPORT(Import),
  INTERFACE(Interface),
  RETURN(Return),
  VARIABLESET(VariableSet),
  VARIABLE(Variable),
}

impl Statements {
  pub fn get_block(&self) -> Option<Block> {
    match self {
      Statements::BLOCK(block) => Some(block.clone()),
      _ => None,
    }
  }

  pub fn get_export(&self) -> Option<Export> {
    match self {
      Statements::EXPORT(export) => Some(export.clone()),
      _ => None,
    }
  }

  pub fn get_expression(&self) -> Option<ExpressionStatement> {
    match self {
      Statements::EXPRESSION(exp) => Some(exp.clone()),
      _ => None,
    }
  }

  pub fn get_function(&self) -> Option<Function> {
    match self {
      Statements::FUNCTION(function) => Some(function.clone()),
      _ => None,
    }
  }

  pub fn get_if_else(&self) -> Option<IfElse> {
    match self {
      Statements::IFELSE(if_else) => Some(if_else.clone()),
      _ => None,
    }
  }

  pub fn get_import(&self) -> Option<Import> {
    match self {
      Statements::IMPORT(import) => Some(import.clone()),
      _ => None,
    }
  }

  pub fn get_interface(&self) -> Option<Interface> {
    match self {
      Statements::INTERFACE(interface) => Some(interface.clone()),
      _ => None,
    }
  }

  pub fn get_return(&self) -> Option<Return> {
    match self {
      Statements::RETURN(return_s) => Some(return_s.clone()),
      _ => None,
    }
  }

  pub fn get_variable_set(&self) -> Option<VariableSet> {
    match self {
      Statements::VARIABLESET(variable_set) => Some(variable_set.clone()),
      _ => None,
    }
  }

  pub fn get_variable(&self) -> Option<Variable> {
    match self {
      Statements::VARIABLE(variable) => Some(variable.clone()),
      _ => None,
    }
  }

  pub fn token(&self) -> Token {
    match self.clone() {
      Statements::BLOCK(block) => block.token,
      Statements::EXPORT(export) => export.token,
      Statements::EXPRESSION(expression) => expression.token,
      Statements::FUNCTION(function) => function.token,
      Statements::IFELSE(if_else) => if_else.token,
      Statements::IMPORT(import) => import.token,
      Statements::INTERFACE(interface) => interface.token,
      Statements::RETURN(return_s) => return_s.token,
      Statements::VARIABLE(variable) => variable.token,
      Statements::VARIABLESET(variable_set) => variable_set.token,
    }
  }

  pub fn string(&self) -> String {
    match self {
      Statements::BLOCK(block) => block.string(),
      Statements::EXPORT(export) => export.string(),
      Statements::EXPRESSION(exp) => exp.string(),
      Statements::FUNCTION(function) => function.string(),
      Statements::IFELSE(if_else) => if_else.string(),
      Statements::IMPORT(import) => import.string(),
      Statements::INTERFACE(interface) => interface.string(),
      Statements::RETURN(return_s) => return_s.string(),
      Statements::VARIABLE(variable) => variable.string(),
      Statements::VARIABLESET(variable_set) => variable_set.string(),
    }
  }
}
