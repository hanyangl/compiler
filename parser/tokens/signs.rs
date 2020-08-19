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

  // Assign
  ASSIGN,
  PLUSASSIGN,
  MINUSASSIGN,
  MULTIPLYASSIGN,
  DIVIDEASSIGN,

  PLUSPLUS,
  MINUSMINUS,

  // Conditions
  EQUAL,
  EQUALTYPE,
  NOTEQUAL,
  NOTEQUALTYPE,

  LESSTHAN,
  LESSOREQUALTHAN,
  GREATERTHAN,
  GREATEROREQUALTHAN,

  AND,
  OR,

  // Operators
  PLUS,
  MINUS,
  MULTIPLY,
  DIVIDE,
  EMPOWERMENT,
  MODULE,

  // Others
  NEGATION,
  ARROW,
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

      // Assign
      "=" => Some(TokenType::new(Signs::ASSIGN)),
      "+=" => Some(TokenType::new(Signs::PLUSASSIGN)),
      "-=" => Some(TokenType::new(Signs::MINUSASSIGN)),
      "*=" => Some(TokenType::new(Signs::MULTIPLYASSIGN)),
      "/=" => Some(TokenType::new(Signs::DIVIDEASSIGN)),

      "++" => Some(TokenType::new(Signs::PLUSPLUS)),
      "--" => Some(TokenType::new(Signs::MINUSMINUS)),

      // Conditions
      "==" => Some(TokenType::new(Signs::EQUAL)),
      "===" => Some(TokenType::new(Signs::EQUALTYPE)),
      "!=" => Some(TokenType::new(Signs::NOTEQUAL)),
      "!==" => Some(TokenType::new(Signs::NOTEQUALTYPE)),

      "<" => Some(TokenType::new(Signs::LESSTHAN)),
      "<=" => Some(TokenType::new(Signs::LESSOREQUALTHAN)),
      ">" => Some(TokenType::new(Signs::GREATERTHAN)),
      ">=" => Some(TokenType::new(Signs::GREATEROREQUALTHAN)),

      "&&" => Some(TokenType::new(Signs::AND)),
      "||" => Some(TokenType::new(Signs::OR)),

      // Maths
      "+" => Some(TokenType::new(Signs::PLUS)),
      "-" => Some(TokenType::new(Signs::MINUS)),
      "*" => Some(TokenType::new(Signs::MULTIPLY)),
      "/" => Some(TokenType::new(Signs::DIVIDE)),
      "**" => Some(TokenType::new(Signs::EMPOWERMENT)),
      "%" => Some(TokenType::new(Signs::MODULE)),

      // Others
      "!" => Some(TokenType::new(Signs::NEGATION)),
      "->" => Some(TokenType::new(Signs::ARROW)),

      // Default
      _ => None,
    }
  }
}
