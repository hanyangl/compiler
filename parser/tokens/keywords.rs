use super::Tokens;

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
  // Variables
  LET,
  CONST,
  AS,

  // Function
  FUNCTION,
  RETURN,

  // Boolean
  TRUE,
  FALSE,

  // Cycles
  FOR,
  IN,
  OF,
  WHILE,
  DO,

  // Conditions
  IF,
  ELSE,
  SWITCH,
  IS,

  // Classes
  PUBLIC,
  PRIVATE,
  PROTECTED,
  INTERNAL,
  READONLY,
  CLASS,
  THIS,

  // Modules
  IMPORT,
  FROM,
  EXPORT,

  // Others
  DEFAULT,
  INTERFACE,
  ENUM,
}

impl Keywords {
  pub fn new(keyword: Keywords) -> Box<Tokens> {
    Box::new(Tokens::KEYWORD(keyword))
  }

  pub fn from_value(value: &str) -> Result<Keywords, ()> {
    match value {
      // Variables
      "let" => Ok(Keywords::LET),
      "const" => Ok(Keywords::CONST),
      "as" => Ok(Keywords::AS),

      // Function
      "func" => Ok(Keywords::FUNCTION),
      "return" => Ok(Keywords::RETURN),

      // Boolean
      "true" => Ok(Keywords::TRUE),
      "false" => Ok(Keywords::FALSE),

      // Cycles
      "for" => Ok(Keywords::FOR),
      "in" => Ok(Keywords::IN),
      "of" => Ok(Keywords::OF),
      "while" => Ok(Keywords::WHILE),
      "do" => Ok(Keywords::DO),

      // Conditions
      "if" => Ok(Keywords::IF),
      "else" => Ok(Keywords::ELSE),
      "switch" => Ok(Keywords::SWITCH),
      "is" => Ok(Keywords::IS),

      // Classes
      "public" => Ok(Keywords::PUBLIC),
      "private" => Ok(Keywords::PRIVATE),
      "protected" => Ok(Keywords::PROTECTED),
      "internal" => Ok(Keywords::INTERNAL),
      "readonly" => Ok(Keywords::READONLY),
      "class" => Ok(Keywords::CLASS),
      "this" => Ok(Keywords::THIS),

      // Modules
      "import" => Ok(Keywords::IMPORT),
      "from" => Ok(Keywords::FROM),
      "export" => Ok(Keywords::EXPORT),

      // Others
      "default" => Ok(Keywords::DEFAULT),
      "interface" => Ok(Keywords::INTERFACE),
      "enum" => Ok(Keywords::ENUM),

      // Default
      _ => Err(()),
    }
  }
}
