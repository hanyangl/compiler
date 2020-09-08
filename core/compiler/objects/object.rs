use super::*;

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
  ANONYMOUSFUNCTION(AnonymousFunction),
  ARRAY(Array),
  BOOLEAN(Boolean),
  BUILTIN(BuiltIn),
  ERROR(Error),
  HASHMAP(HashMap),
  NULL(Null),
  NUMBER(Number),
  RETURN(ReturnO),
  STRING(StringO),
}

impl Objects {
  pub fn get_anonymous_function(self) -> Option<AnonymousFunction> {
    match self {
      Objects::ANONYMOUSFUNCTION(anonymous_function) => Some(anonymous_function),
      _ => None,
    }
  }

  pub fn get_array(self) -> Option<Array> {
    match self {
      Objects::ARRAY(array) => Some(array),
      _ => None,
    }
  }

  pub fn get_boolean(self) -> Option<Boolean> {
    match self {
      Objects::BOOLEAN(boolean) => Some(boolean),
      _ => None,
    }
  }

  pub fn expect_boolean(self, value: bool) -> bool {
    match self {
      Objects::BOOLEAN(boolean) => boolean.value == value,
      _ => false,
    }
  }

  pub fn get_builtin(self) -> Option<BuiltIn> {
    match self {
      Objects::BUILTIN(builtin) => Some(builtin),
      _ => None,
    }
  }

  pub fn get_error(self) -> Option<Error> {
    match self {
      Objects::ERROR(error) => Some(error),
      _ => None,
    }
  }

  pub fn get_hashmap(self) -> Option<HashMap> {
    match self {
      Objects::HASHMAP(hashmap) => Some(hashmap),
      _ => None,
    }
  }

  pub fn get_null(self) -> Option<Null> {
    match self {
      Objects::NULL(null) => Some(null),
      _ => None,
    }
  }

  pub fn get_number(self) -> Option<Number> {
    match self {
      Objects::NUMBER(number) => Some(number),
      _ => None,
    }
  }

  pub fn get_return(self) -> Option<ReturnO> {
    match self {
      Objects::RETURN(return_o) => Some(return_o),
      _ => None,
    }
  }

  pub fn get_string(self) -> Option<StringO> {
    match self {
      Objects::STRING(string) => Some(string),
      _ => None,
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
      Objects::ANONYMOUSFUNCTION(anonymous_function) => anonymous_function.string(),
      Objects::ARRAY(array) => array.string(),
      Objects::BOOLEAN(boolean) => boolean.string(),
      Objects::BUILTIN(builtin) => builtin.string(),
      Objects::HASHMAP(hashmap) => hashmap.string(),
      Objects::NULL(null) => null.string(),
      Objects::NUMBER(number) => number.string(),
      Objects::RETURN(return_o) => return_o.string(),
      Objects::STRING(string) => string.string(),
      _ => String::new(),
    }
  }
}
