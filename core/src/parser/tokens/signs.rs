use super::{TokenType, Tokens};

#[derive(Debug, Clone, PartialEq)]
pub enum Signs {
  // Delimeters
  COMMA,
  COLON,
  SEMICOLON,

  LEFTPARENTHESES,
  RIGHTPARENTHESES,

  LEFTBRACKET,
  RIGHTBRACKET,

  LEFTBRACE,
  RIGHTBRACE,

  // Others
  ASSIGN,
}

impl TokenType for Signs {
  fn new(sign: Signs) -> Box<Tokens> {
    Box::new(Tokens::SIGN(sign))
  }

  fn from_value(value: String) -> Option<Box<Tokens>> {
    match value.as_str() {
      // Delimeters
      "," => Some(TokenType::new(Signs::COMMA)),
      ":" => Some(TokenType::new(Signs::COLON)),
      ";" => Some(TokenType::new(Signs::SEMICOLON)),

      "(" => Some(TokenType::new(Signs::LEFTPARENTHESES)),
      ")" => Some(TokenType::new(Signs::RIGHTPARENTHESES)),

      "[" => Some(TokenType::new(Signs::LEFTBRACKET)),
      "]" => Some(TokenType::new(Signs::RIGHTBRACKET)),

      "{" => Some(TokenType::new(Signs::LEFTBRACE)),
      "}" => Some(TokenType::new(Signs::RIGHTBRACE)),

      // Others
      "=" => Some(TokenType::new(Signs::ASSIGN)),

      // Default
      _ => None,
    }
  }
}
