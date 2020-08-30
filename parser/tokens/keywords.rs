use super::{TokenType, Tokens};

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
  // Variables
  LET,
  CONST,

  // Function
  FUNCTION,
  RETURN,

  // Boolean
  TRUE,
  FALSE,

  // Alias
  AS,

  // Modules
  IMPORT,
  EXPORT,
  FROM,
}

impl TokenType for Keywords {
  fn new(keyword: Keywords) -> Box<Tokens> {
    Box::new(Tokens::KEYWORD(keyword))
  }

  fn from_value(value: String) -> Option<Box<Tokens>> {
    match value.as_str() {
      // Variables
      "let" => Some(TokenType::new(Keywords::LET)),
      "const" => Some(TokenType::new(Keywords::CONST)),

      // Function
      "function" => Some(TokenType::new(Keywords::FUNCTION)),
      "return" => Some(TokenType::new(Keywords::RETURN)),

      // Bolean
      "true" => Some(TokenType::new(Keywords::TRUE)),
      "false" => Some(TokenType::new(Keywords::FALSE)),

      // Alias
      "as" => Some(TokenType::new(Keywords::AS)),

      // Modules
      "import" => Some(TokenType::new(Keywords::IMPORT)),
      "export" => Some(TokenType::new(Keywords::EXPORT)),
      "from" => Some(TokenType::new(Keywords::FROM)),

      // Default
      _ => None,
    }
  }
}
