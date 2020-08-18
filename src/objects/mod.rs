pub mod boolean;
pub mod error;
pub mod function;
pub mod integer;
pub mod null;
pub mod return_o;
pub mod string;

use crate::data::Types;
use crate::expressions::Expression;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
  NULL,
  ERROR,

  INTEGER,
  BOOLEAN,
  STRING,

  RETURN,

  FUNCTION,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashKey {
  pub object_type: ObjectType,
  pub value: u64,
}

pub trait Hashable {
  fn hashkey(self) -> HashKey;
}

pub trait Object {
  fn object_type(&self) -> ObjectType;
  fn string(self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Objects {
  NULL(null::Null),
  ERROR(error::Error),

  INTEGER(integer::Integer),
  BOOLEAN(boolean::Boolean),
  STRING(string::StringO),

  RETURN(return_o::Return),

  FUNCTION(function::Function),
}

impl Objects {
  /// Get empty object for a data type.
  pub fn empty(data_type: Types) -> Box<Objects> {
    match data_type {
      // String
      Types::STRING => string::StringO::new(String::new()),

      // Integer
      Types::NUMBER => integer::Integer::new(0),

      // Boolean
      Types::BOOLEAN => boolean::Boolean::new(false),

      // Void
      Types::VOID => function::Function::new(Expression::new()),

      // Default
      _ => null::Null::new(),
    }
  }

  /// Get null object.
  pub fn get_null(self) -> Option<null::Null> {
    match self {
      Objects::NULL(null) => Some(null),
      _ => None,
    }
  }

  /// Get error object.
  pub fn get_error(self) -> Option<error::Error> {
    match self {
      Objects::ERROR(error) => Some(error),
      _ => None,
    }
  }

  /// Get integer object.
  pub fn get_integer(self) -> Option<integer::Integer> {
    match self {
      Objects::INTEGER(integer) => Some(integer),
      _ => None,
    }
  }

  /// Get boolean object.
  pub fn get_boolean(self) -> Option<boolean::Boolean> {
    match self {
      Objects::BOOLEAN(boolean) => Some(boolean),
      _ => None,
    }
  }

  /// Get string object.
  pub fn get_string(self) -> Option<string::StringO> {
    match self {
      Objects::STRING(string) => Some(string),
      _ => None,
    }
  }

  /// Get return object.
  pub fn get_return(self) -> Option<return_o::Return> {
    match self {
      Objects::RETURN(return_o) => Some(return_o),
      _ => None,
    }
  }

  /// Get function object.
  pub fn get_function(self) -> Option<function::Function> {
    match self {
      Objects::FUNCTION(function) => Some(function),
      _ => None,
    }
  }

  /// Get the object type.
  pub fn object_type(self) -> ObjectType {
    match self {
      Objects::NULL(null) => null.object_type(),
      Objects::ERROR(error) => error.object_type(),

      Objects::INTEGER(integer) => integer.object_type(),
      Objects::BOOLEAN(boolean) => boolean.object_type(),
      Objects::STRING(string) => string.object_type(),

      Objects::RETURN(return_o) => return_o.object_type(),

      Objects::FUNCTION(function) => function.object_type(),
    }
  }

  /// Get the object string.
  pub fn string(self) -> String {
    match self {
      Objects::NULL(null) => null.string(),
      Objects::ERROR(error) => error.string(),

      Objects::INTEGER(integer) => integer.string(),
      Objects::BOOLEAN(boolean) => boolean.string(),
      Objects::STRING(string) => string.string(),

      Objects::RETURN(return_o) => return_o.string(),

      Objects::FUNCTION(function) => function.string(),
    }
  }
}
