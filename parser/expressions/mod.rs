mod anonymous_function;
mod argument;
mod array;
mod boolean;
mod call;
mod hashmap;
mod identifier;
mod infix;
mod method;
mod number;
mod parser;
mod prefix;
mod string;
pub mod types;

pub use anonymous_function::AnonymousFunction;
pub use argument::Argument;
pub use array::{ArrayType, Array};
pub use boolean::Boolean;
pub use call::Call;
pub use hashmap::{HashMapItem, HashMap};
pub use identifier::Identifier;
pub use infix::Infix;
pub use method::Method;
pub use number::Number;
pub use parser::parse;
pub use prefix::Prefix;
pub use string::StringE;

use crate::tokens::Token;

pub trait Expression {
  /// Create a new empty expression.
  fn new() -> Self;

  /// Create a new expression from a token.
  fn from_token(token: Token) -> Self;

  /// Get the expression value.
  fn string(self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expressions {
  ANONYMOUSFUNCTION(AnonymousFunction),
  ARGUMENT(Argument),
  ARRAY(Array),
  BOOLEAN(Boolean),
  CALL(Call),
  HASHMAP(HashMap),
  IDENTIFIER(Identifier),
  INFIX(Infix),
  METHOD(Method),
  NUMBER(Number),
  PREFIX(Prefix),
  STRING(StringE),
}

impl Expressions {
  pub fn get_anonymous_function(self) -> Option<AnonymousFunction> {
    match self {
      Expressions::ANONYMOUSFUNCTION(anonymous_function) => Some(anonymous_function),
      _ => None,
    }
  }

  pub fn is_anonymous_function(self) -> bool {
    match self {
      Expressions::ANONYMOUSFUNCTION(_) => true,
      _ => false,
    }
  }

  pub fn get_argument(self) -> Option<Argument> {
    match self {
      Expressions::ARGUMENT(argument) => Some(argument),
      _ => None,
    }
  }

  pub fn get_array(self) -> Option<Array> {
    match self {
      Expressions::ARRAY(array) => Some(array),
      _ => None,
    }
  }

  pub fn get_boolean(self) -> Option<Boolean> {
    match self {
      Expressions::BOOLEAN(boolean) => Some(boolean),
      _ => None,
    }
  }

  pub fn get_call(self) -> Option<Call> {
    match self {
      Expressions::CALL(call) => Some(call),
      _ => None,
    }
  }

  pub fn is_call(self) -> bool {
    match self {
      Expressions::CALL(_) => true,
      _ => false,
    }
  }

  pub fn get_hashmap(self) -> Option<HashMap> {
    match self {
      Expressions::HASHMAP(hashmap) => Some(hashmap),
      _ => None,
    }
  }

  pub fn is_hashmap(self) -> bool {
    match self {
      Expressions::HASHMAP(_) => true,
      _ => false,
    }
  }

  pub fn get_identifier(self) -> Option<Identifier> {
    match self {
      Expressions::IDENTIFIER(identifier) => Some(identifier),
      _ => None,
    }
  }

  pub fn get_infix(self) -> Option<Infix> {
    match self {
      Expressions::INFIX(infix) => Some(infix),
      _ => None,
    }
  }

  pub fn get_method(self) -> Option<Method> {
    match self {
      Expressions::METHOD(method) => Some(method),
      _ => None,
    }
  }

  pub fn is_method(self) -> bool {
    match self {
      Expressions::METHOD(_) => true,
      _ => false,
    }
  }

  pub fn get_number(self) -> Option<Number> {
    match self {
      Expressions::NUMBER(number) => Some(number),
      _ => None,
    }
  }

  pub fn is_number(self) -> bool {
    match self {
      Expressions::NUMBER(_) => true,
      _ => false,
    }
  }

  pub fn get_prefix(self) -> Option<Prefix> {
    match self {
      Expressions::PREFIX(prefix) => Some(prefix),
      _ => None,
    }
  }

  pub fn is_prefix(self) -> bool {
    match self {
      Expressions::PREFIX(_) => true,
      _ => false,
    }
  }

  pub fn get_string(self) -> Option<StringE> {
    match self {
      Expressions::STRING(string) => Some(string),
      _ => None,
    }
  }

  pub fn token(self) -> Token {
    match self {
      Expressions::ANONYMOUSFUNCTION(anonymous_function) => anonymous_function.token,
      Expressions::ARGUMENT(argument) => argument.token,
      Expressions::ARRAY(array) => array.token,
      Expressions::BOOLEAN(boolean) => boolean.token,
      Expressions::CALL(call) => call.token,
      Expressions::HASHMAP(hashmap) => hashmap.token,
      Expressions::IDENTIFIER(identifier) => identifier.token,
      Expressions::INFIX(infix) => infix.token,
      Expressions::METHOD(method) => method.token,
      Expressions::NUMBER(number) => number.token,
      Expressions::PREFIX(prefix) => prefix.token,
      Expressions::STRING(string) => string.token,
    }
  }

  pub fn string(self) -> String {
    match self {
      Expressions::ANONYMOUSFUNCTION(anonymous_function) => anonymous_function.string(),
      Expressions::ARGUMENT(argument) => argument.string(),
      Expressions::ARRAY(array) => array.string(),
      Expressions::BOOLEAN(boolean) => boolean.string(),
      Expressions::CALL(call) => call.string(),
      Expressions::HASHMAP(hashmap) => hashmap.string(),
      Expressions::IDENTIFIER(identifier) => identifier.string(),
      Expressions::INFIX(infix) => infix.string(),
      Expressions::METHOD(method) => method.string(),
      Expressions::NUMBER(number) => number.string(),
      Expressions::PREFIX(prefix) => prefix.string(),
      Expressions::STRING(string) => string.string(),
    }
  }
}
