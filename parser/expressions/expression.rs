use crate::tokens::Token;

use super::*;

pub trait Expression {
  /// Create a new empty expression.
  fn new() -> Self;

  /// Create a new expression from a token.
  fn from_token(token: Token) -> Self;

  /// Get the token.
  fn get_token(&self) -> Token;

  /// Get the expression value.
  fn string(&self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expressions {
  ANONYMOUSFUNCTION(AnonymousFunction),
  ARGUMENT(Argument),
  ARRAY(Array),
  ARRAYINDEX(ArrayIndex),
  BOOLEAN(Boolean),
  CALL(Call),
  HASHMAP(HashMap),
  IDENTIFIER(Identifier),
  INFIX(Infix),
  NULL(Null),
  NUMBER(Number),
  PREFIX(Prefix),
  STRING(StringE),
}

impl Expressions {
  pub fn get_anonymous_function(&self) -> Option<AnonymousFunction> {
    match self {
      Expressions::ANONYMOUSFUNCTION(anonymous_function) => Some(anonymous_function.clone()),
      _ => None,
    }
  }

  pub fn get_argument(&self) -> Option<Argument> {
    match self {
      Expressions::ARGUMENT(argument) => Some(argument.clone()),
      _ => None,
    }
  }

  pub fn get_array(&self) -> Option<Array> {
    match self {
      Expressions::ARRAY(array) => Some(array.clone()),
      _ => None,
    }
  }

  pub fn get_array_index(&self) -> Option<ArrayIndex> {
    match self {
      Expressions::ARRAYINDEX(array_index) => Some(array_index.clone()),
      _ => None,
    }
  }

  pub fn get_boolean(&self) -> Option<Boolean> {
    match self {
      Expressions::BOOLEAN(boolean) => Some(boolean.clone()),
      _ => None,
    }
  }
  pub fn get_call(&self) -> Option<Call> {
    match self {
      Expressions::CALL(call) => Some(call.clone()),
      _ => None,
    }
  }

  pub fn get_hashmap(&self) -> Option<HashMap> {
    match self {
      Expressions::HASHMAP(hashmap) => Some(hashmap.clone()),
      _ => None,
    }
  }

  pub fn get_identifier(&self) -> Option<Identifier> {
    match self {
      Expressions::IDENTIFIER(identifier) => Some(identifier.clone()),
      _ => None,
    }
  }

  pub fn get_infix(&self) -> Option<Infix> {
    match self {
      Expressions::INFIX(infix) => Some(infix.clone()),
      _ => None,
    }
  }

  pub fn get_null(&self) -> Option<Null> {
    match self {
      Expressions::NULL(null) => Some(null.clone()),
      _ => None,
    }
  }

  pub fn get_number(&self) -> Option<Number> {
    match self {
      Expressions::NUMBER(number) => Some(number.clone()),
      _ => None,
    }
  }

  pub fn get_prefix(&self) -> Option<Prefix> {
    match self {
      Expressions::PREFIX(prefix) => Some(prefix.clone()),
      _ => None,
    }
  }

  pub fn get_string(&self) -> Option<StringE> {
    match self {
      Expressions::STRING(string) => Some(string.clone()),
      _ => None,
    }
  }

  pub fn token(&self) -> Token {
    match self {
      Expressions::ANONYMOUSFUNCTION(anonymous_function) => anonymous_function.get_token(),
      Expressions::ARGUMENT(argument) => argument.get_token(),
      Expressions::ARRAY(array) => array.get_token(),
      Expressions::ARRAYINDEX(array_index) => array_index.get_token(),
      Expressions::BOOLEAN(boolean) => boolean.get_token(),
      Expressions::CALL(call) => call.get_token(),
      Expressions::HASHMAP(hashmap) => hashmap.get_token(),
      Expressions::IDENTIFIER(identifier) => identifier.get_token(),
      Expressions::INFIX(infix) => infix.get_token(),
      Expressions::NULL(null) => null.get_token(),
      Expressions::NUMBER(number) => number.get_token(),
      Expressions::PREFIX(prefix) => prefix.get_token(),
      Expressions::STRING(string) => string.get_token(),
    }
  }

  pub fn string(&self) -> String {
    match self {
      Expressions::ANONYMOUSFUNCTION(anonymous_function) => anonymous_function.string(),
      Expressions::ARGUMENT(argument) => argument.string(),
      Expressions::ARRAY(array) => array.string(),
      Expressions::ARRAYINDEX(array_index) => array_index.string(),
      Expressions::BOOLEAN(boolean) => boolean.string(),
      Expressions::CALL(call) => call.string(),
      Expressions::HASHMAP(hashmap) => hashmap.string(),
      Expressions::IDENTIFIER(identifier) => identifier.string(),
      Expressions::INFIX(infix) => infix.string(),
      Expressions::NULL(null) => null.string(),
      Expressions::NUMBER(number) => number.string(),
      Expressions::PREFIX(prefix) => prefix.string(),
      Expressions::STRING(string) => string.string(),
    }
  }
}
