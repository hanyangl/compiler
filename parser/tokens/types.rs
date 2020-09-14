mod array;
mod function;
mod hashmap;

pub use array::Array;
pub use function::Function;
pub use hashmap::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
  NULL,
  STRING,
  NUMBER,
  BOOLEAN,
  VOID,

  ARRAY(Array),
  FUNCTION(Function),
  HASHMAP(HashMap),
}

impl Types {
  pub fn get_array(&self) -> Option<Array> {
    match self {
      Types::ARRAY(array) => Some(array.clone()),
      _ => None,
    }
  }

  pub fn get_function(&self) -> Option<Function> {
    match self {
      Types::FUNCTION(function) => Some(function.clone()),
      _ => None,
    }
  }

  pub fn get_hashmap(&self) -> Option<HashMap> {
    match self {
      Types::HASHMAP(hashmap) => Some(hashmap.clone()),
      _ => None,
    }
  }

  pub fn from_value(value: &str) -> Result<Types, ()> {
    // Parse array.
    if let Ok(array) = Array::from_value(value) {
      return Ok(Types::ARRAY(array));
    }

    // Parse hashmaps.
    if let Ok(hashmap) = HashMap::from_value(value) {
      return Ok(Types::HASHMAP(hashmap));
    }

    // Parse functions.
    if let Ok(function) = Function::from_value(value) {
      return Ok(Types::FUNCTION(function));
    }

    match value {
      "null" => Ok(Types::NULL),
      "string" => Ok(Types::STRING),
      "number" => Ok(Types::NUMBER),
      "boolean" => Ok(Types::BOOLEAN),
      "void" => Ok(Types::VOID),

      // Default
      _ => Err(()),
    }
  }
}
