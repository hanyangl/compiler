use crate::tokens::Token;

use super::*;

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
  CLASS(Class),
  CLASSCONSTRUCTOR(ClassConstructor),
  CLASSMETHOD(ClassMethod),
  EXPORT(Export),
  EXPRESSION(ExpressionStatement),
  FUNCTION(Function),
  IFELSE(IfElse),
  IMPORT(Import),
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

  pub fn get_class(self) -> Option<Class> {
    match self {
      Statements::CLASS(class) => Some(class),
      _ => None,
    }
  }

  pub fn get_class_constructor(self) -> Option<ClassConstructor> {
    match self {
      Statements::CLASSCONSTRUCTOR(class_constructor) => Some(class_constructor),
      _ => None,
    }
  }

  pub fn get_class_method(self) -> Option<ClassMethod> {
    match self {
      Statements::CLASSMETHOD(class_method) => Some(class_method),
      _ => None,
    }
  }

  pub fn get_export(self) -> Option<Export> {
    match self {
      Statements::EXPORT(export) => Some(export),
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

  pub fn get_if_else(self) -> Option<IfElse> {
    match self {
      Statements::IFELSE(if_else) => Some(if_else),
      _ => None,
    }
  }

  pub fn get_import(self) -> Option<Import> {
    match self {
      Statements::IMPORT(import) => Some(import),
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

  pub fn token(self) -> Token {
    match self {
      Statements::BLOCK(block) => block.token,
      Statements::CLASS(class) => class.token,
      Statements::CLASSCONSTRUCTOR(class_constructor) => class_constructor.token,
      Statements::CLASSMETHOD(class_method) => class_method.token,
      Statements::EXPORT(export) => export.token,
      Statements::EXPRESSION(expression) => expression.token,
      Statements::FUNCTION(function) => function.token,
      Statements::IFELSE(if_else) => if_else.token,
      Statements::IMPORT(import) => import.token,
      Statements::RETURN(return_s) => return_s.token,
      Statements::VARIABLE(variable) => variable.token,
      Statements::VARIABLESET(variable_set) => variable_set.token,
    }
  }

  pub fn string(self) -> String {
    match self {
      Statements::BLOCK(block) => block.string(),
      Statements::CLASS(class) => class.string(),
      Statements::CLASSCONSTRUCTOR(class_constructor) => class_constructor.string(),
      Statements::CLASSMETHOD(class_method) => class_method.string(),
      Statements::EXPORT(export) => export.string(),
      Statements::EXPRESSION(exp) => exp.string(),
      Statements::FUNCTION(function) => function.string(),
      Statements::IFELSE(if_else) => if_else.string(),
      Statements::IMPORT(import) => import.string(),
      Statements::RETURN(return_s) => return_s.string(),
      Statements::VARIABLE(variable) => variable.string(),
      Statements::VARIABLESET(variable_set) => variable_set.string(),
    }
  }
}
