#[derive(Debug)]
#[derive(PartialEq)]
pub enum Types {
  UNDEFINED,
  NULL,

  STRING,
  NUMBER,

  BOOLEAN,
  TRUE,
  FALSE,

  ARRAY,

  NONE,
}

pub fn get_type(value: &String) -> Types {
  match value.as_str() {
    "undefined" => Types::UNDEFINED,
    "null" => Types::NULL,

    "string" => Types::STRING,
    "number" => Types::NUMBER,

    "boolean" => Types::BOOLEAN,
    "true" => Types::TRUE,
    "false" => Types::FALSE,

    _ => Types::NONE,
  }
}
