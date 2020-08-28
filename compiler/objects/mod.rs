mod array;
mod boolean;
mod error;
mod hashmap;
mod null;
mod number;
mod return_o;
mod string;

pub use array::Array;
pub use boolean::Boolean;
pub use error::Error;
pub use hashmap::HashMap;
pub use null::Null;
pub use number::Number;
pub use return_o::ReturnO;
pub use string::StringO;

#[derive(Debug, Clone, PartialEq)]
pub struct HashKey {
  pub value: f64,
}

pub trait Hashable {
  fn get_hashkey(self) -> HashKey;
}

pub trait Object {
  fn string(self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Objects {
  ARRAY(Array),
  BOOLEAN(Boolean),
  ERROR(Error),
  HASHMAP(HashMap),
  NULL(Null),
  NUMBER(Number),
  RETURN(ReturnO),
  STRING(StringO),
}

impl Objects {
  pub fn get_array(self) -> Option<Array> {
    match self {
      Objects::ARRAY(array) => Some(array),
      _ => None,
    }
  }

  pub fn is_array(self) -> bool {
    match self {
      Objects::ARRAY(_) => true,
      _ => false,
    }
  }

  pub fn get_boolean(self) -> Option<Boolean> {
    match self {
      Objects::BOOLEAN(boolean) => Some(boolean),
      _ => None,
    }
  }

  pub fn is_boolean(self) -> bool {
    match self {
      Objects::BOOLEAN(_) => true,
      _ => false,
    }
  }

  pub fn expect_boolean(self, value: bool) -> bool {
    match self {
      Objects::BOOLEAN(boolean) => boolean.value == value,
      _ => false,
    }
  }

  pub fn get_error(self) -> Option<Error> {
    match self {
      Objects::ERROR(error) => Some(error),
      _ => None,
    }
  }

  pub fn is_error(self) -> bool {
    match self {
      Objects::ERROR(_) => true,
      _ => false,
    }
  }

  pub fn get_hashmap(self) -> Option<HashMap> {
    match self {
      Objects::HASHMAP(hashmap) => Some(hashmap),
      _ => None,
    }
  }

  pub fn is_hashmap(self) -> bool {
    match self {
      Objects::HASHMAP(_) => true,
      _ => false,
    }
  }

  pub fn get_null(self) -> Option<Null> {
    match self {
      Objects::NULL(null) => Some(null),
      _ => None,
    }
  }

  pub fn is_null(self) -> bool {
    match self {
      Objects::NULL(_) => true,
      _ => false,
    }
  }

  pub fn get_number(self) -> Option<Number> {
    match self {
      Objects::NUMBER(number) => Some(number),
      _ => None,
    }
  }

  pub fn is_number(self) -> bool {
    match self {
      Objects::NUMBER(_) => true,
      _ => false,
    }
  }

  pub fn get_return(self) -> Option<ReturnO> {
    match self {
      Objects::RETURN(return_o) => Some(return_o),
      _ => None,
    }
  }

  pub fn is_return(self) -> bool {
    match self {
      Objects::RETURN(_) => true,
      _ => false,
    }
  }

  pub fn get_string(self) -> Option<StringO> {
    match self {
      Objects::STRING(string) => Some(string),
      _ => None,
    }
  }

  pub fn is_string(self) -> bool {
    match self {
      Objects::STRING(_) => true,
      _ => false,
    }
  }

  pub fn get_hashkey(self) -> Option<HashKey> {
    match self {
      Objects::BOOLEAN(boolean) => Some(boolean.get_hashkey()),
      Objects::NUMBER(number) => Some(number.get_hashkey()),
      Objects::STRING(string) => Some(string.get_hashkey()),
      _ => None,
    }
  }

  pub fn string(self) -> String {
    match self {
      Objects::ARRAY(array) => array.string(),
      Objects::BOOLEAN(boolean) => boolean.string(),
      Objects::ERROR(error) => error.string(),
      Objects::HASHMAP(hashmap) => hashmap.string(),
      Objects::NULL(null) => null.string(),
      Objects::NUMBER(number) => number.string(),
      Objects::RETURN(return_o) => return_o.string(),
      Objects::STRING(string) => string.string(),
    }
  }
}
