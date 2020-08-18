use super::{TokenType, Tokens};

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
  // Basic
  STRING,
  NUMBER,
  BOOLEAN,

  // Function
  VOID,
}

impl TokenType for Types {
  fn new(data_type: Types) -> Box<Tokens> {
    Box::new(Tokens::TYPE(data_type))
  }

  fn from_value(value: String) -> Option<Box<Tokens>> {
    match value.as_str() {
      // Basic
      "string" => Some(TokenType::new(Types::STRING)),
      "number" => Some(TokenType::new(Types::NUMBER)),
      "boolean" => Some(TokenType::new(Types::BOOLEAN)),

      // Function
      "void" => Some(TokenType::new(Types::VOID)),

      // Default
      _ => None,
    }
  }
}
