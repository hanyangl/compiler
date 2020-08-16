pub mod boolean;
pub mod call;
pub mod function;
pub mod identifier;
pub mod if_else;
pub mod infix;
pub mod integer;
pub mod method;
pub mod parameter;
mod parser;
pub mod prefix;
pub mod string;

use crate::data;
pub use parser::parse;

pub trait Expression {
  /// Create a new empty expression.
  fn new() -> Self;

  /// Create a new expression from a token.
  fn from_token(token: &data::Token) -> Self;

  /// Get the expression value.
  fn string(self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expressions {
  BOOLEAN(boolean::Boolean),
  CALL(call::Call),
  FUNCTION(function::Function),
  IDENTIFIER(identifier::Identifier),
  IFELSE(if_else::IfElse),
  INFIX(infix::Infix),
  INTEGER(integer::Integer),
  METHOD(method::Method),
  PARAMETER(parameter::Parameter),
  PREFIX(prefix::Prefix),
  STRING(string::StringE),
}

impl Expressions {
  /// Get the boolean expression.
  pub fn get_boolean(self) -> Option<boolean::Boolean> {
    match self {
      Expressions::BOOLEAN(boolean) => Some(boolean),
      _ => None,
    }
  }

  /// Get the call expression.
  pub fn get_call(self) -> Option<call::Call> {
    match self {
      Expressions::CALL(call) => Some(call),
      _ => None,
    }
  }

  /// Get the function expression.
  pub fn get_function(self) -> Option<function::Function> {
    match self {
      Expressions::FUNCTION(function) => Some(function),
      _ => None,
    }
  }

  /// Get the identifier expression.
  pub fn get_identifier(self) -> Option<identifier::Identifier> {
    match self {
      Expressions::IDENTIFIER(identifier) => Some(identifier),
      _ => None,
    }
  }

  /// Get the if-else expression.
  pub fn get_ifelse(self) -> Option<if_else::IfElse> {
    match self {
      Expressions::IFELSE(ifelse) => Some(ifelse),
      _ => None,
    }
  }

  /// Get the infix expression.
  pub fn get_infix(self) -> Option<infix::Infix> {
    match self {
      Expressions::INFIX(infix) => Some(infix),
      _ => None,
    }
  }

  /// Get the integer expression.
  pub fn get_integer(self) -> Option<integer::Integer> {
    match self {
      Expressions::INTEGER(integer) => Some(integer),
      _ => None,
    }
  }

  /// Get the method expression.
  pub fn get_method(self) -> Option<method::Method> {
    match self {
      Expressions::METHOD(method) => Some(method),
      _ => None,
    }
  }

  /// Get the parameter expression.
  pub fn get_parameter(self) -> Option<parameter::Parameter> {
    match self {
      Expressions::PARAMETER(parameter) => Some(parameter),
      _ => None,
    }
  }

  /// Get the prefix expression.
  pub fn get_prefix(self) -> Option<prefix::Prefix> {
    match self {
      Expressions::PREFIX(prefix) => Some(prefix),
      _ => None,
    }
  }

  /// Get the string expression.
  pub fn get_string(self) -> Option<string::StringE> {
    match self {
      Expressions::STRING(string) => Some(string),
      _ => None,
    }
  }

  /// Get the expression token.
  pub fn token(self) -> data::Token {
    match self {
      Expressions::BOOLEAN(boolean) => boolean.token,
      Expressions::CALL(call) => call.token,
      Expressions::FUNCTION(function) => function.token,
      Expressions::IDENTIFIER(identifier) => identifier.token,
      Expressions::IFELSE(ifelse) => ifelse.token,
      Expressions::INFIX(infix) => infix.token,
      Expressions::INTEGER(integer) => integer.token,
      Expressions::METHOD(method) => method.token,
      Expressions::PARAMETER(parameter) => parameter.data_type,
      Expressions::PREFIX(prefix) => prefix.token,
      Expressions::STRING(string) => string.token,
    }
  }

  /// Get the expression as string.
  pub fn string(self) -> String {
    match self {
      Expressions::BOOLEAN(boolean) => boolean.string(),
      Expressions::CALL(call) => call.string(),
      Expressions::FUNCTION(function) => function.string(),
      Expressions::IDENTIFIER(identifier) => identifier.string(),
      Expressions::IFELSE(ifelse) => ifelse.string(),
      Expressions::INFIX(infix) => infix.string(),
      Expressions::INTEGER(integer) => integer.string(),
      Expressions::METHOD(method) => method.string(),
      Expressions::PARAMETER(parameter) => parameter.string(),
      Expressions::PREFIX(prefix) => prefix.string(),
      Expressions::STRING(string) => string.string(),
    }
  }
}
